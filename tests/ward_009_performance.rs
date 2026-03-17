use std::time::Instant;

use kmd_engine::batch::evaluator::batch_evaluate;
use kmd_engine::borger_generator;
use kmd_engine::dependency_graph::DependencyGraph;
use kmd_engine::rule_engine::{Rule, RuleParams};
use kmd_engine::rules::boerne_ydelse::BoerneYdelse;
use kmd_engine::rules::boligstoette::Boligstoette;
use kmd_engine::rules::kontanthjaelp::Kontanthjaelp;
use kmd_engine::scenario::incremental::{compute_dirty_set, incremental_evaluate};
use kmd_engine::scenario::param_mapping::{ParamId, ParamRuleMapping};
use kmd_engine::scenario::scenario::Scenario;
use kmd_engine::wasm_api::Engine;

fn build_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(Kontanthjaelp),
        Box::new(Boligstoette),
        Box::new(BoerneYdelse),
    ]
}

fn build_graph(rules: &[Box<dyn Rule>]) -> DependencyGraph {
    let refs: Vec<&dyn Rule> = rules.iter().map(|r| r.as_ref()).collect();
    DependencyGraph::build(&refs).unwrap()
}

fn is_release() -> bool {
    !cfg!(debug_assertions)
}

// ── Test 1: Batch eval 100K under 30ms (release) ──
#[test]
fn test_batch_100k_30ms() {
    if !is_release() {
        eprintln!("SKIP: test_batch_100k_30ms requires --release");
        return;
    }
    let store = borger_generator::generate(42, 100_000);
    let rules = build_rules();
    let graph = build_graph(&rules);
    let params = RuleParams::default();

    // Warmup
    let _ = batch_evaluate(&store, &rules, &graph, &params);

    let t0 = Instant::now();
    let result = batch_evaluate(&store, &rules, &graph, &params);
    let elapsed = t0.elapsed();

    assert_eq!(result.count, 100_000);
    assert!(
        elapsed.as_millis() < 30,
        "batch 100K should be under 30ms in release, took {}ms", elapsed.as_millis()
    );
    eprintln!("bench: batch_100k = {:.2}ms", elapsed.as_secs_f64() * 1000.0);
}

// ── Test 2: Incremental 100K under 15ms (release) ──
#[test]
fn test_incremental_100k_15ms() {
    if !is_release() {
        eprintln!("SKIP: test_incremental_100k_15ms requires --release");
        return;
    }
    let store = borger_generator::generate(42, 100_000);
    let rules = build_rules();
    let graph = build_graph(&rules);
    let params = RuleParams::default();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);
    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasisEnlig, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);

    // Warmup
    let _ = incremental_evaluate(&store, &rules, &graph, &baseline, &scenario.params, &dirty);

    let t0 = Instant::now();
    let _ = incremental_evaluate(&store, &rules, &graph, &baseline, &scenario.params, &dirty);
    let elapsed = t0.elapsed();

    assert!(
        elapsed.as_millis() < 15,
        "incremental 100K should be under 15ms in release, took {}ms", elapsed.as_millis()
    );
    eprintln!("bench: incremental_100k = {:.2}ms", elapsed.as_secs_f64() * 1000.0);
}

// ── Test 3: Batch eval 500K under 150ms (stretch) ──
#[test]
fn test_batch_500k_150ms() {
    if !is_release() {
        eprintln!("SKIP: test_batch_500k_150ms requires --release");
        return;
    }
    let store = borger_generator::generate(42, 500_000);
    let rules = build_rules();
    let graph = build_graph(&rules);
    let params = RuleParams::default();

    let _ = batch_evaluate(&store, &rules, &graph, &params);

    let t0 = Instant::now();
    let result = batch_evaluate(&store, &rules, &graph, &params);
    let elapsed = t0.elapsed();

    assert_eq!(result.count, 500_000);
    assert!(
        elapsed.as_millis() < 150,
        "batch 500K should be under 150ms in release, took {}ms", elapsed.as_millis()
    );
    eprintln!("bench: batch_500k = {:.2}ms", elapsed.as_secs_f64() * 1000.0);
}

// ── Test 4: Incremental 500K under 80ms (stretch) ──
#[test]
fn test_incremental_500k_80ms() {
    if !is_release() {
        eprintln!("SKIP: test_incremental_500k_80ms requires --release");
        return;
    }
    let store = borger_generator::generate(42, 500_000);
    let rules = build_rules();
    let graph = build_graph(&rules);
    let params = RuleParams::default();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);
    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasisEnlig, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);

    let _ = incremental_evaluate(&store, &rules, &graph, &baseline, &scenario.params, &dirty);

    let t0 = Instant::now();
    let _ = incremental_evaluate(&store, &rules, &graph, &baseline, &scenario.params, &dirty);
    let elapsed = t0.elapsed();

    assert!(
        elapsed.as_millis() < 80,
        "incremental 500K should be under 80ms in release, took {}ms", elapsed.as_millis()
    );
    eprintln!("bench: incremental_500k = {:.2}ms", elapsed.as_secs_f64() * 1000.0);
}

// ── Test 5: WASM→JS transfer proxy (JSON serialization) under 5ms ──
#[test]
fn test_wasm_js_transfer_5ms() {
    if !is_release() {
        eprintln!("SKIP: test_wasm_js_transfer_5ms requires --release");
        return;
    }
    let mut engine = Engine::new();
    let _ = engine.init();
    let _ = engine.apply_scenario(0, 13_050.0);

    // Warmup
    let _ = engine.get_geo_data();

    let t0 = Instant::now();
    let geo = engine.get_geo_data();
    let elapsed_geo = t0.elapsed();

    let t1 = Instant::now();
    let stats = engine.get_filtered_stats(None);
    let elapsed_stats = t1.elapsed();

    assert!(!geo.is_empty());
    assert!(!stats.is_empty());
    assert!(
        elapsed_geo.as_millis() < 5,
        "geo JSON serialization should be under 5ms, took {}ms", elapsed_geo.as_millis()
    );
    assert!(
        elapsed_stats.as_millis() < 5,
        "stats JSON serialization should be under 5ms, took {}ms", elapsed_stats.as_millis()
    );
    eprintln!("bench: geo_serialize = {:.2}ms, stats_serialize = {:.2}ms",
        elapsed_geo.as_secs_f64() * 1000.0,
        elapsed_stats.as_secs_f64() * 1000.0,
    );
}

// ── Test 7: End-to-end 100K under 100ms (init excluded, scenario cycle) ──
#[test]
fn test_e2e_100k_100ms() {
    if !is_release() {
        eprintln!("SKIP: test_e2e_100k_100ms requires --release");
        return;
    }
    let mut engine = Engine::new();
    let _ = engine.init();

    // Warmup
    let _ = engine.apply_scenario(0, 13_050.0);

    let t0 = Instant::now();
    let scenario_json = engine.apply_scenario(0, 13_500.0);
    let geo_json = engine.get_geo_data();
    let stats_json = engine.get_filtered_stats(None);
    let top_json = engine.get_top_affected(10);
    let elapsed = t0.elapsed();

    assert!(!scenario_json.is_empty());
    assert!(!geo_json.is_empty());
    assert!(!stats_json.is_empty());
    assert!(!top_json.is_empty());
    assert!(
        elapsed.as_millis() < 100,
        "e2e scenario cycle should be under 100ms for 100K, took {}ms", elapsed.as_millis()
    );
    eprintln!("bench: e2e_100k = {:.2}ms", elapsed.as_secs_f64() * 1000.0);
}

// ── Test 8: Memory under 50MB for 100K ──
#[test]
fn test_memory_100k() {
    let store = borger_generator::generate(42, 100_000);
    let rules = build_rules();
    let graph = build_graph(&rules);
    let params = RuleParams::default();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let store_bytes = store.heap_size_bytes();
    let result_bytes = baseline.count * std::mem::size_of::<kmd_engine::batch::types::CompactBorgerResult>();
    let total_bytes = store_bytes + result_bytes;
    let total_mb = total_bytes as f64 / (1024.0 * 1024.0);

    assert!(
        total_mb < 50.0,
        "100K memory should be under 50MB, measured {:.2}MB", total_mb
    );
    eprintln!("bench: memory_100k = {:.2}MB (store={:.2}MB, results={:.2}MB)",
        total_mb,
        store_bytes as f64 / (1024.0 * 1024.0),
        result_bytes as f64 / (1024.0 * 1024.0),
    );
}

// ── Test 9: find_by_id O(1) after optimization ──
#[test]
fn test_find_by_id_o1() {
    let store = borger_generator::generate(42, 100_000);

    let t0 = Instant::now();
    for id in 1..=100_000_u32 {
        let idx = store.find_by_id(id);
        assert!(idx.is_some(), "borger_id {} should be found", id);
        assert_eq!(idx.unwrap(), (id - 1) as usize);
    }
    let elapsed = t0.elapsed();

    assert!(store.find_by_id(0).is_none(), "borger_id 0 should not exist");
    assert!(store.find_by_id(100_001).is_none(), "borger_id 100001 should not exist");

    eprintln!("bench: find_by_id x100K = {:.2}ms", elapsed.as_secs_f64() * 1000.0);
}
