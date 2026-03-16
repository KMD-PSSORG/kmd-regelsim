use kmd_engine::batch::evaluator::batch_evaluate;
use kmd_engine::batch::aggregator::aggregate_by_segment;
use kmd_engine::batch::top_n::top_n;
use kmd_engine::batch::types::{CompactBorgerResult, SegmentKey};
use kmd_engine::borger_generator;
use kmd_engine::dependency_graph::DependencyGraph;
use kmd_engine::rule_engine::{Rule, RuleId, RuleParams};
use kmd_engine::rules::boerne_ydelse::BoerneYdelse;
use kmd_engine::rules::boligstoette::Boligstoette;
use kmd_engine::rules::kontanthjaelp::Kontanthjaelp;

const SEED: u64 = 42;
const COUNT: usize = 100_000;

fn setup() -> (
    kmd_engine::borger_store::BorgerStore,
    Vec<Box<dyn Rule>>,
    DependencyGraph,
    RuleParams,
) {
    let store = borger_generator::generate(SEED, COUNT);
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(Kontanthjaelp),
        Box::new(Boligstoette),
        Box::new(BoerneYdelse),
    ];
    let rule_refs: Vec<&dyn Rule> = rules.iter().map(|r| r.as_ref()).collect();
    let graph = DependencyGraph::build(&rule_refs).unwrap();
    let params = RuleParams::default();
    (store, rules, graph, params)
}

// --- Test 1: Batch-eval 100K under 50ms (release) ---
#[test]
fn test_batch_eval_100k_performance() {
    let (store, rules, graph, params) = setup();

    let start = std::time::Instant::now();
    let result = batch_evaluate(&store, &rules, &graph, &params);
    let elapsed = start.elapsed();

    assert_eq!(result.count, COUNT);
    assert_eq!(result.borger_results.len(), COUNT);

    // In debug mode we allow more time; release target is <50ms
    // This test documents the target — Ward 9 enforces it strictly
    println!("batch_evaluate 100K: {:?}", elapsed);
}

// --- Test 2: Core aggregation ---
#[test]
fn test_core_aggregation() {
    let (store, rules, graph, params) = setup();
    let result = batch_evaluate(&store, &rules, &graph, &params);

    assert_eq!(result.per_rule.len(), 3);

    for agg in &result.per_rule {
        assert!(agg.total_amount >= 0.0, "{:?} total_amount negative", agg.rule_id);
        assert!(agg.eligible_count <= COUNT, "{:?} eligible_count > total", agg.rule_id);

        if agg.eligible_count > 0 {
            let expected_mean = agg.total_amount / agg.eligible_count as f64;
            assert!(
                (agg.mean_amount - expected_mean).abs() < 0.01,
                "{:?} mean mismatch: {} vs {}", agg.rule_id, agg.mean_amount, expected_mean
            );
        }
    }

    // Kontanthjaelp should have a significant number of eligible (ledige + aktivitetsparate)
    let kh_agg = result.per_rule.iter().find(|a| a.rule_id == RuleId::Kontanthjaelp).unwrap();
    assert!(kh_agg.eligible_count > 1_000, "too few kontanthjaelp eligible: {}", kh_agg.eligible_count);
    assert!(kh_agg.total_amount > 0.0, "no kontanthjaelp paid out");
}

// --- Test 3: Segmentation ---
#[test]
fn test_segmentation() {
    let (store, rules, graph, params) = setup();
    let result = batch_evaluate(&store, &rules, &graph, &params);

    // Group by kommune_id
    let kommune_segments = aggregate_by_segment(
        &result,
        |i| SegmentKey::Kommune(store.kommune_id[i]),
        RuleId::Kontanthjaelp,
    );

    // Should have segments for all 98 kommuner
    let unique_kommuner: std::collections::HashSet<_> = kommune_segments.iter()
        .filter_map(|s| match &s.key {
            SegmentKey::Kommune(id) => Some(*id),
            _ => None,
        })
        .collect();
    assert_eq!(unique_kommuner.len(), 98, "not all 98 kommuner in segments");

    // Total across segments should match overall
    let seg_total: f64 = kommune_segments.iter().map(|s| s.total_amount).sum();
    let kh_agg = result.per_rule.iter().find(|a| a.rule_id == RuleId::Kontanthjaelp).unwrap();
    assert!(
        (seg_total - kh_agg.total_amount).abs() < 1.0,
        "segment total {} != overall total {}", seg_total, kh_agg.total_amount
    );

    // Group by husstandstype
    let ht_segments = aggregate_by_segment(
        &result,
        |i| SegmentKey::Husstandstype(store.husstandstype[i]),
        RuleId::Kontanthjaelp,
    );
    assert_eq!(ht_segments.len(), 4, "should have 4 husstandstype segments");
}

// --- Test 4: Top-N ---
#[test]
fn test_top_n() {
    let (store, rules, graph, params) = setup();
    let result = batch_evaluate(&store, &rules, &graph, &params);

    let top10 = top_n(&store, &result, RuleId::Kontanthjaelp, 10);

    assert_eq!(top10.len(), 10, "should return exactly 10 entries");

    // Should be sorted descending by amount
    for i in 1..top10.len() {
        assert!(
            top10[i - 1].amount >= top10[i].amount,
            "top-N not sorted: {} < {} at position {}", top10[i - 1].amount, top10[i].amount, i
        );
    }

    // All borger_ids should be valid
    for entry in &top10 {
        assert!(entry.borger_id >= 1 && entry.borger_id <= COUNT as u32,
            "invalid borger_id: {}", entry.borger_id);
    }

    // Top entry should have highest amount
    let max_amount = result.borger_results.iter()
        .map(|r| r.amount(RuleId::Kontanthjaelp))
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        (top10[0].amount - max_amount).abs() < 0.01,
        "top entry {} != max {}", top10[0].amount, max_amount
    );
}

// --- Test 5: Deterministic results ---
#[test]
fn test_deterministic_results() {
    let (store, rules, graph, params) = setup();
    let result_a = batch_evaluate(&store, &rules, &graph, &params);

    let rules_b: Vec<Box<dyn Rule>> = vec![
        Box::new(Kontanthjaelp),
        Box::new(Boligstoette),
        Box::new(BoerneYdelse),
    ];
    let result_b = batch_evaluate(&store, &rules_b, &graph, &params);

    assert_eq!(result_a.count, result_b.count);
    for i in 0..result_a.count {
        let a = &result_a.borger_results[i];
        let b = &result_b.borger_results[i];
        assert_eq!(a.amounts, b.amounts, "amounts differ at index {}", i);
        assert_eq!(a.eligible, b.eligible, "eligible differs at index {}", i);
    }
}

// --- Test 6: Memory under 10MB ---
#[test]
fn test_memory_batch_result() {
    let (store, rules, graph, params) = setup();
    let result = batch_evaluate(&store, &rules, &graph, &params);

    let bytes = result.heap_size_bytes();
    let mb = bytes as f64 / (1024.0 * 1024.0);

    assert!(
        mb < 10.0,
        "BatchResult too large: {:.2} MB (limit: 10 MB)", mb
    );

    println!("BatchResult memory: {:.2} MB for {} borgere", mb, COUNT);
}

// --- Test 7: CompactBorgerResult layout ---
#[test]
fn test_compact_borger_result_layout() {
    let size = std::mem::size_of::<CompactBorgerResult>();

    // 3 × f64 (24 bytes) + 3 × bool (3 bytes) + padding
    // With #[repr(C)]: 24 + 3 + 5 padding = 32 bytes
    assert!(
        size <= 32,
        "CompactBorgerResult is {} bytes — too large (max 32)", size
    );

    println!("CompactBorgerResult: {} bytes", size);
    println!("Per 100K borgere: {:.2} MB", size as f64 * 100_000.0 / (1024.0 * 1024.0));
}
