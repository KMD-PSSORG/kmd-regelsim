use kmd_engine::borger_generator;
use kmd_engine::dependency_graph::DependencyGraph;
use kmd_engine::error::EngineError;
use kmd_engine::eval_context::EvalContext;
use kmd_engine::rule_engine::{Rule, RuleId, RuleParams, RuleResult};
use kmd_engine::rules::boerne_ydelse::BoerneYdelse;
use kmd_engine::rules::boligstoette::Boligstoette;
use kmd_engine::rules::kontanthjaelp::Kontanthjaelp;
use kmd_engine::types::Husstandstype;

const SEED: u64 = 42;

fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(Kontanthjaelp),
        Box::new(Boligstoette),
        Box::new(BoerneYdelse),
    ]
}

fn run_rules_for_borger(
    rules: &[Box<dyn Rule>],
    graph: &DependencyGraph,
    store: &kmd_engine::borger_store::BorgerStore,
    index: usize,
    params: &RuleParams,
) -> EvalContext {
    let borger = store.view(index);
    let mut ctx = EvalContext::new();
    for &rule_id in graph.evaluation_order() {
        let rule = rules.iter().find(|r| r.id() == rule_id).unwrap();
        let result = rule.evaluate(&borger, &ctx, params);
        ctx.insert(result);
    }
    ctx
}

fn borger_is_ledig(borger: &kmd_engine::borger_view::BorgerView) -> bool {
    borger.beskaeftigelsesstatus == kmd_engine::types::Beskaeftigelsesstatus::Ledig
        || borger.beskaeftigelsesstatus == kmd_engine::types::Beskaeftigelsesstatus::Aktivitetsparat
}

// --- Test 1: Single rule produces a RuleResult ---
#[test]
fn test_single_rule_evaluation() {
    let store = borger_generator::generate(SEED, 100);
    let borger = store.view(0);
    let params = RuleParams::default();
    let ctx = EvalContext::new();

    let kh = Kontanthjaelp;
    let result = kh.evaluate(&borger, &ctx, &params);

    assert_eq!(result.rule_id, RuleId::Kontanthjaelp);
    assert!(result.amount >= 0.0, "amount should be non-negative");
}

// --- Test 2: Dependency order ---
#[test]
fn test_dependency_order() {
    let rules = all_rules();
    let rule_refs: Vec<&dyn Rule> = rules.iter().map(|r| r.as_ref()).collect();
    let graph = DependencyGraph::build(&rule_refs).expect("should build without cycles");

    let order = graph.evaluation_order();

    let kh_pos = order.iter().position(|&id| id == RuleId::Kontanthjaelp).unwrap();
    let bs_pos = order.iter().position(|&id| id == RuleId::Boligstoette).unwrap();

    assert!(
        kh_pos < bs_pos,
        "Kontanthjaelp (pos {}) must come before Boligstoette (pos {})",
        kh_pos, bs_pos
    );
}

// --- Test 3: Circular dependency returns Err ---
#[test]
fn test_circular_dependency_result() {
    struct FakeCircularRule;
    impl Rule for FakeCircularRule {
        fn id(&self) -> RuleId { RuleId::Kontanthjaelp }
        fn dependencies(&self) -> &[RuleId] { &[RuleId::Boligstoette] }
        fn evaluate(&self, _: &kmd_engine::borger_view::BorgerView, _: &EvalContext, _: &RuleParams) -> RuleResult {
            unreachable!()
        }
    }

    let circular = FakeCircularRule;
    let bs = Boligstoette;
    let rules: Vec<&dyn Rule> = vec![&circular, &bs];

    let result = DependencyGraph::build(&rules);

    assert!(result.is_err(), "should detect cycle");
    match result.unwrap_err() {
        EngineError::CyclicDependency { cycle } => {
            assert!(!cycle.is_empty(), "cycle description should not be empty");
        }
        other => panic!("expected CyclicDependency, got: {:?}", other),
    }
}

// --- Test 4: All 3 rules produce results ---
#[test]
fn test_all_rules_individual() {
    let store = borger_generator::generate(SEED, 1_000);
    let params = RuleParams::default();

    let rules = all_rules();
    let rule_refs: Vec<&dyn Rule> = rules.iter().map(|r| r.as_ref()).collect();
    let graph = DependencyGraph::build(&rule_refs).unwrap();

    for i in 0..100 {
        let ctx = run_rules_for_borger(&rules, &graph, &store, i, &params);

        assert!(ctx.get(&RuleId::Kontanthjaelp).is_some(), "missing kontanthjaelp for borger {}", i);
        assert!(ctx.get(&RuleId::Boligstoette).is_some(), "missing boligstoette for borger {}", i);
        assert!(ctx.get(&RuleId::BoerneYdelse).is_some(), "missing boerneydelse for borger {}", i);

        for rule_id in [RuleId::Kontanthjaelp, RuleId::Boligstoette, RuleId::BoerneYdelse] {
            let r = ctx.get(&rule_id).unwrap();
            assert!(r.amount >= 0.0, "negative amount for {:?} borger {}", rule_id, i);
        }
    }
}

// --- Test 5: Hand-calculated kontanthjaelp cases ---
#[test]
fn test_kontanthjaelp_manual_cases() {
    let params = RuleParams::default();
    let ctx = EvalContext::new();
    let kh = Kontanthjaelp;

    let store = borger_generator::generate(SEED, 10_000);

    let mut found_enlig_uden = false;
    let mut found_enlig_med = false;
    let mut found_par_uden = false;
    let mut found_par_med = false;

    for i in 0..store.len() {
        let borger = store.view(i);
        let result = kh.evaluate(&borger, &ctx, &params);

        if !borger_is_ledig(&borger) {
            continue;
        }

        match (borger.husstandstype, borger.antal_boern) {
            // Case 1: Enlig uden boern, ledig
            (Husstandstype::Enlig, 0) if !found_enlig_uden => {
                assert!(result.eligible, "enlig ledig uden boern should be eligible");
                assert_eq!(result.amount, params.kontanthjaelp_basis_enlig,
                    "enlig uden boern should get basis: {}", result.amount);
                found_enlig_uden = true;
            }
            // Case 2: Enlig forsoerger med boern, ledig
            (Husstandstype::EnligForsoerger, n) if n > 0 && !found_enlig_med => {
                assert!(result.eligible, "enlig forsoerger ledig should be eligible");
                let expected = (params.kontanthjaelp_basis_enlig
                    + n as f64 * params.forsoergertillaeg_per_barn)
                    .min(params.kontanthjaelpsloft_enlig);
                assert!((result.amount - expected).abs() < 0.01,
                    "enlig forsoerger: got {}, expected {}", result.amount, expected);
                found_enlig_med = true;
            }
            // Case 3: Par uden boern, ledig
            (Husstandstype::ParUdenBoern, 0) if !found_par_uden => {
                assert!(result.eligible, "par uden boern ledig should be eligible");
                assert_eq!(result.amount, params.kontanthjaelp_basis_par,
                    "par uden boern should get par-basis: {}", result.amount);
                found_par_uden = true;
            }
            // Case 4: Par med boern, ledig
            (Husstandstype::ParMedBoern, n) if n > 0 && !found_par_med => {
                assert!(result.eligible, "par med boern ledig should be eligible");
                let expected = (params.kontanthjaelp_basis_par
                    + n as f64 * params.forsoergertillaeg_per_barn)
                    .min(params.kontanthjaelpsloft_par);
                assert!((result.amount - expected).abs() < 0.01,
                    "par med boern: got {}, expected {}", result.amount, expected);
                found_par_med = true;
            }
            _ => {}
        }

        if found_enlig_uden && found_enlig_med && found_par_uden && found_par_med {
            break;
        }
    }

    assert!(found_enlig_uden, "could not find enlig ledig uden boern in 10K");
    assert!(found_enlig_med, "could not find enlig forsoerger ledig med boern in 10K");
    assert!(found_par_uden, "could not find par uden boern ledig in 10K");
    assert!(found_par_med, "could not find par med boern ledig in 10K");
}

// --- Test 6: Configurable parameters ---
#[test]
fn test_configurable_params() {
    let store = borger_generator::generate(SEED, 1_000);
    let kh = Kontanthjaelp;
    let ctx = EvalContext::new();

    let idx = (0..store.len())
        .find(|&i| {
            let b = store.view(i);
            b.husstandstype == Husstandstype::Enlig && borger_is_ledig(&b)
        })
        .expect("should find an enlig ledig borger");

    let borger = store.view(idx);

    let params_default = RuleParams::default();
    let result_default = kh.evaluate(&borger, &ctx, &params_default);

    let mut params_modified = RuleParams::default();
    params_modified.kontanthjaelp_basis_enlig += 500.0;
    let result_modified = kh.evaluate(&borger, &ctx, &params_modified);

    assert!(
        (result_modified.amount - result_default.amount - 500.0).abs() < 0.01,
        "parameter change of +500 should increase amount by 500, got delta: {}",
        result_modified.amount - result_default.amount
    );
}

// --- Test 7: EvalContext accumulates results ---
#[test]
fn test_context_accumulates() {
    let store = borger_generator::generate(SEED, 1_000);
    let params = RuleParams::default();
    let rules = all_rules();
    let rule_refs: Vec<&dyn Rule> = rules.iter().map(|r| r.as_ref()).collect();
    let graph = DependencyGraph::build(&rule_refs).unwrap();

    let ctx = run_rules_for_borger(&rules, &graph, &store, 0, &params);

    assert_eq!(ctx.results().len(), 3, "should have results for all 3 rules");
    assert!(ctx.get(&RuleId::Kontanthjaelp).is_some());
    assert!(ctx.get(&RuleId::Boligstoette).is_some());
    assert!(ctx.get(&RuleId::BoerneYdelse).is_some());

    // Boligstoette evaluated after kontanthjaelp — produces valid result
    let bs_result = ctx.get(&RuleId::Boligstoette).unwrap();
    assert!(bs_result.amount >= 0.0);
}

// --- Test 8: No explanations in batch results ---
#[test]
fn test_no_explanations_in_batch() {
    let size = std::mem::size_of::<RuleResult>();
    assert!(
        size <= 24,
        "RuleResult too large: {} bytes (should be <= 24). No explanation strings in batch.",
        size
    );

    let result = RuleResult {
        rule_id: RuleId::Kontanthjaelp,
        amount: 12_550.0,
        eligible: true,
    };
    assert_eq!(result.rule_id, RuleId::Kontanthjaelp);
    assert_eq!(result.amount, 12_550.0);
    assert!(result.eligible);
}
