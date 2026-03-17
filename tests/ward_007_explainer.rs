use kmd_engine::batch::evaluator::batch_evaluate;
use kmd_engine::borger_generator;
use kmd_engine::dependency_graph::DependencyGraph;
use kmd_engine::explainer::case_explainer::explain_case;
use kmd_engine::rule_engine::{Rule, RuleId, RuleParams};
use kmd_engine::rules::boerne_ydelse::BoerneYdelse;
use kmd_engine::rules::boligstoette::Boligstoette;
use kmd_engine::rules::kontanthjaelp::Kontanthjaelp;
use kmd_engine::scenario::incremental::{compute_dirty_set, incremental_evaluate};
use kmd_engine::scenario::param_mapping::ParamRuleMapping;
use kmd_engine::scenario::param_mapping::ParamId;
use kmd_engine::scenario::scenario::Scenario;

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

// ── Test 1: Case detail by borger_id ──
#[test]
fn test_case_detail_by_borger_id() {
    let (store, rules, graph, params) = setup();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let explanation = explain_case(
        &store, &rules, &graph, &params, None, &baseline, None, 1, 0.03,
    ).unwrap();

    assert_eq!(explanation.borger_id, 1);
    assert_eq!(explanation.rules.len(), 3, "should have trace for all 3 rules");
    assert!(!explanation.husstandstype.is_empty(), "husstandstype should be set");
    assert!(!explanation.beskaeftigelse.is_empty(), "beskaeftigelse should be set");

    for rule in &explanation.rules {
        assert!(!rule.rule.is_empty(), "rule name should be set");
        assert!(!rule.explanation.is_empty(), "explanation should be human-readable");
    }
}

// ── Test 2: Human-readable explanations ──
#[test]
fn test_human_readable_explanations() {
    let (store, rules, graph, params) = setup();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    // Find a borger who is eligible for kontanthjaelp (ledig/aktivitetsparat)
    let borger_id = (1..=COUNT as u32).find(|&id| {
        let idx = store.find_by_id(id).unwrap();
        baseline.borger_results[idx].is_eligible(RuleId::Kontanthjaelp)
    }).expect("should find an eligible borger");

    let explanation = explain_case(
        &store, &rules, &graph, &params, None, &baseline, None, borger_id, 0.03,
    ).unwrap();

    let kh = explanation.rules.iter().find(|r| r.rule == "kontanthjaelp").unwrap();
    assert!(kh.eligible, "borger should be eligible for kontanthjaelp");
    assert!(kh.baseline_amount > 0.0, "should have a positive amount");

    // Explanation should contain human-readable text, not just numbers
    assert!(kh.explanation.contains("kr"), "explanation should mention kr: {}", kh.explanation);
    assert!(kh.explanation.len() > 20, "explanation should be descriptive: {}", kh.explanation);
}

// ── Test 3: Diff view (baseline vs scenario) ──
#[test]
fn test_diff_view() {
    let (store, rules, graph, params) = setup();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasis, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);
    let scenario_result = incremental_evaluate(
        &store, &rules, &graph, &baseline, &scenario.params, &dirty,
    );

    // Find a borger affected by the change
    let borger_id = (1..=COUNT as u32).find(|&id| {
        let idx = store.find_by_id(id).unwrap();
        let base = baseline.borger_results[idx].amount(RuleId::Kontanthjaelp);
        let scen = scenario_result.borger_results[idx].amount(RuleId::Kontanthjaelp);
        (base - scen).abs() > 0.01
    }).expect("should find an affected borger");

    let explanation = explain_case(
        &store, &rules, &graph, &params,
        Some(&scenario.params), &baseline, Some(&scenario_result),
        borger_id, 0.03,
    ).unwrap();

    let kh = explanation.rules.iter().find(|r| r.rule == "kontanthjaelp").unwrap();
    assert!(kh.delta.abs() > 0.01, "delta should be non-zero for affected borger");
    assert_ne!(kh.baseline_amount, kh.scenario_amount,
        "baseline and scenario amounts should differ");
}

// ── Test 4: Highlight largest delta ──
#[test]
fn test_highlight_largest_delta() {
    let (store, rules, graph, params) = setup();
    let mapping = ParamRuleMapping::new();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    let scenario = Scenario::new(&params, ParamId::KontanthjaelpBasis, 13_050.0);
    let dirty = compute_dirty_set(&scenario.overrides, &mapping, &graph, &rules);
    let scenario_result = incremental_evaluate(
        &store, &rules, &graph, &baseline, &scenario.params, &dirty,
    );

    let borger_id = (1..=COUNT as u32).find(|&id| {
        let idx = store.find_by_id(id).unwrap();
        let base = baseline.borger_results[idx].amount(RuleId::Kontanthjaelp);
        let scen = scenario_result.borger_results[idx].amount(RuleId::Kontanthjaelp);
        (base - scen).abs() > 0.01
    }).unwrap();

    let explanation = explain_case(
        &store, &rules, &graph, &params,
        Some(&scenario.params), &baseline, Some(&scenario_result),
        borger_id, 0.03,
    ).unwrap();

    // The rule with the largest |delta| should be identifiable
    let max_delta_rule = explanation.rules.iter()
        .max_by(|a, b| a.delta.abs().partial_cmp(&b.delta.abs()).unwrap())
        .unwrap();
    assert!(max_delta_rule.delta.abs() > 0.0, "largest delta should be non-zero");
    // kontanthjaelp should have the largest delta (it's the changed rule)
    assert_eq!(max_delta_rule.rule, "kontanthjaelp",
        "kontanthjaelp should have largest delta, got {}", max_delta_rule.rule);
}

// ── Test 5: Outlier with configurable threshold ──
#[test]
fn test_outlier_configurable_threshold() {
    let (store, rules, graph, params) = setup();
    let baseline = batch_evaluate(&store, &rules, &graph, &params);

    // With a very wide threshold (50%), almost every eligible borger should get warnings
    let wide = explain_case(
        &store, &rules, &graph, &params, None, &baseline, None, 1, 0.50,
    ).unwrap();
    let wide_warnings: usize = wide.rules.iter().map(|r| r.threshold_warnings.len()).sum();

    // With a very narrow threshold (0.1%), few should get warnings
    let narrow = explain_case(
        &store, &rules, &graph, &params, None, &baseline, None, 1, 0.001,
    ).unwrap();
    let narrow_warnings: usize = narrow.rules.iter().map(|r| r.threshold_warnings.len()).sum();

    // Wide threshold should produce at least as many warnings as narrow
    assert!(wide_warnings >= narrow_warnings,
        "wider threshold ({}) should produce >= warnings than narrow ({})",
        wide_warnings, narrow_warnings);
}
