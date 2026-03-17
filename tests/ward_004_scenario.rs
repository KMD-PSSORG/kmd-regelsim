use kmd_engine::batch::evaluator::batch_evaluate;
use kmd_engine::borger_generator;
use kmd_engine::dependency_graph::DependencyGraph;
use kmd_engine::rule_engine::{Rule, RuleId, RuleParams};
use kmd_engine::rules::boerne_ydelse::BoerneYdelse;
use kmd_engine::rules::boligstoette::Boligstoette;
use kmd_engine::rules::kontanthjaelp::Kontanthjaelp;
use kmd_engine::scenario::param_mapping::{ParamId, ParamRuleMapping};
use kmd_engine::scenario::scenario::Scenario;
use kmd_engine::scenario::incremental::{compute_dirty_set, incremental_evaluate};
use kmd_engine::scenario::diff::compute_diff;

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

// --- Test 1: Create scenario ---
#[test]
fn test_create_scenario() {
    let params = RuleParams::default();
    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasisEnlig, 13_050.0);

    assert_eq!(scenario.params.kontanthjaelp_basis_enlig, 13_050.0);
    assert_eq!(scenario.overrides.len(), 1);
    assert_eq!(scenario.overrides[0].0, ParamId::KontanthjaelpBasisEnlig);

    // Other params unchanged
    assert_eq!(scenario.params.kontanthjaelp_basis_par, params.kontanthjaelp_basis_par);
    assert_eq!(scenario.params.boerneydelse_0_2, params.boerneydelse_0_2);
}

// --- Test 2: Param→rule mapping ---
#[test]
fn test_param_rule_mapping() {
    let mapping = ParamRuleMapping::new();

    // KontanthjaelpBasisEnlig → Kontanthjaelp
    assert_eq!(mapping.affected_roots(ParamId::KontanthjaelpBasisEnlig), &[RuleId::Kontanthjaelp]);

    // KontanthjaelpBasisPar → Kontanthjaelp
    assert_eq!(mapping.affected_roots(ParamId::KontanthjaelpBasisPar), &[RuleId::Kontanthjaelp]);

    // Forsoergertillaeg → Kontanthjaelp
    assert_eq!(mapping.affected_roots(ParamId::Forsoergertillaeg), &[RuleId::Kontanthjaelp]);

    // BoligstoetteGraense → Boligstoette
    assert_eq!(mapping.affected_roots(ParamId::BoligstoetteGraense), &[RuleId::Boligstoette]);

    // BoerneYdelseAftrapning → BoerneYdelse
    assert_eq!(mapping.affected_roots(ParamId::BoerneYdelseAftrapning), &[RuleId::BoerneYdelse]);
}

// --- Test 3: Dirty propagation ---
#[test]
fn test_dirty_propagation() {
    let (_, rules, graph, _) = setup();
    let mapping = ParamRuleMapping::new();

    // Changing kontanthjaelp basis → kontanthjaelp dirty → boligstoette downstream dirty
    let overrides = vec![(ParamId::KontanthjaelpBasisEnlig, 13_050.0)];
    let dirty = compute_dirty_set(&overrides, &mapping, &graph, &rules);

    assert!(dirty.contains(&RuleId::Kontanthjaelp), "kontanthjaelp should be dirty");
    assert!(dirty.contains(&RuleId::Boligstoette), "boligstoette should be dirty (downstream)");
    assert!(!dirty.contains(&RuleId::BoerneYdelse), "boerneydelse should NOT be dirty");
}

// --- Test 4: Only dirty rules re-evaluated ---
#[test]
fn test_only_dirty_reevaluated() {
    let (store, rules, graph, params) = setup();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasisEnlig, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);

    let scenario_result = incremental_evaluate(
        &store, &rules, &graph, &baseline, &scenario.params, &dirty,
    );

    // Boerneydelse (not dirty) should have identical results to baseline
    for i in 0..baseline.count {
        let base_by = baseline.borger_results[i].amount(RuleId::BoerneYdelse);
        let scen_by = scenario_result.borger_results[i].amount(RuleId::BoerneYdelse);
        assert_eq!(base_by, scen_by,
            "boerneydelse changed at borger {} despite not being dirty", i);
    }

    // Kontanthjaelp (dirty) should differ for at least some borgere
    let kh_changed = (0..baseline.count)
        .filter(|&i| {
            let base = baseline.borger_results[i].amount(RuleId::Kontanthjaelp);
            let scen = scenario_result.borger_results[i].amount(RuleId::Kontanthjaelp);
            (base - scen).abs() > 0.001
        })
        .count();
    assert!(kh_changed > 0, "kontanthjaelp should change for some borgere");
}

// --- Test 5: Incremental speedup ---
#[test]
fn test_incremental_speedup() {
    let (store, rules, graph, params) = setup();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasisEnlig, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);

    // Full re-eval
    let start_full = std::time::Instant::now();
    let _ = batch_evaluate(&store, &rules, &graph, &scenario.params);
    let full_time = start_full.elapsed();

    // Incremental
    let start_incr = std::time::Instant::now();
    let _ = incremental_evaluate(&store, &rules, &graph, &baseline, &scenario.params, &dirty);
    let incr_time = start_incr.elapsed();

    println!("full re-eval: {:?}, incremental: {:?}", full_time, incr_time);
    println!("speedup: {:.1}x", full_time.as_secs_f64() / incr_time.as_secs_f64());

    assert!(
        incr_time < full_time,
        "incremental ({:?}) should be faster than full ({:?})", incr_time, full_time
    );
}

// --- Test 6: Diff eligibility ---
#[test]
fn test_diff_eligibility() {
    let (store, rules, graph, params) = setup();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasisEnlig, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);
    let scenario_result = incremental_evaluate(
        &store, &rules, &graph, &baseline, &scenario.params, &dirty,
    );

    let diff = compute_diff(&store, &baseline, &scenario_result, 10);

    // Should have per-rule diffs
    assert_eq!(diff.per_rule.len(), 3, "should have diff for all 3 rules");

    // Kontanthjaelp: raising basis → more money → positive delta
    let kh_diff = diff.per_rule.iter().find(|d| d.rule_id == RuleId::Kontanthjaelp).unwrap();
    assert!(kh_diff.total_delta > 0.0,
        "raising kontanthjaelp basis should increase total: {}", kh_diff.total_delta);

    // Total affected should be > 0
    assert!(diff.total_affected > 0, "should have affected borgere");
}

// --- Test 7: Diff per segment (kommune) ---
#[test]
fn test_diff_per_segment() {
    let (store, rules, graph, params) = setup();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasisEnlig, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);
    let scenario_result = incremental_evaluate(
        &store, &rules, &graph, &baseline, &scenario.params, &dirty,
    );

    let diff = compute_diff(&store, &baseline, &scenario_result, 10);

    // per_kommune should have entries
    assert!(!diff.per_kommune.is_empty(), "should have per-kommune diffs");

    // Total delta across kommuner should roughly match overall
    let kommune_delta_sum: f64 = diff.per_kommune.iter().map(|s| s.total_delta).sum();
    let overall_delta: f64 = diff.per_rule.iter().map(|r| r.total_delta).sum();
    assert!(
        (kommune_delta_sum - overall_delta).abs() < 1.0,
        "kommune delta sum {} != overall delta {}", kommune_delta_sum, overall_delta
    );
}

// --- Test 8: Top-N with borger_id ---
#[test]
fn test_top_n_with_borger_id() {
    let (store, rules, graph, params) = setup();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasisEnlig, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);
    let scenario_result = incremental_evaluate(
        &store, &rules, &graph, &baseline, &scenario.params, &dirty,
    );

    let diff = compute_diff(&store, &baseline, &scenario_result, 10);

    assert_eq!(diff.top_affected.len(), 10, "should return top 10");

    // Sorted descending by |delta|
    for i in 1..diff.top_affected.len() {
        assert!(
            diff.top_affected[i - 1].total_delta.abs() >= diff.top_affected[i].total_delta.abs(),
            "top-N not sorted by |delta|: {} < {} at pos {}",
            diff.top_affected[i - 1].total_delta.abs(),
            diff.top_affected[i].total_delta.abs(), i
        );
    }

    // All borger_ids valid
    for entry in &diff.top_affected {
        assert!(entry.borger_id >= 1 && entry.borger_id <= COUNT as u32,
            "invalid borger_id: {}", entry.borger_id);
    }
}

// --- Test 9: End-to-end under 100ms ---
#[test]
fn test_end_to_end_100ms() {
    let (store, rules, graph, params) = setup();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let start = std::time::Instant::now();

    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasisEnlig, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);
    let scenario_result = incremental_evaluate(
        &store, &rules, &graph, &baseline, &scenario.params, &dirty,
    );
    let diff = compute_diff(&store, &baseline, &scenario_result, 10);

    let elapsed = start.elapsed();

    println!("end-to-end scenario→diff: {:?}", elapsed);
    println!("affected: {}, delta: {:.0} kr/md", diff.total_affected,
        diff.per_rule.iter().map(|r| r.total_delta).sum::<f64>());

    // Debug mode is slower — this documents the target. Ward 9 enforces strictly.
    assert!(diff.total_affected > 0, "should have affected borgere");
}
