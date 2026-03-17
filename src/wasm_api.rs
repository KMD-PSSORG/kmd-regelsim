use serde::Serialize;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

use crate::batch::evaluator::batch_evaluate;
use crate::batch::types::BatchResult;
use crate::borger_generator;
use crate::borger_store::BorgerStore;
use crate::dependency_graph::DependencyGraph;
use crate::rule_engine::{Rule, RuleId, RuleParams};
use crate::rules::boerne_ydelse::BoerneYdelse;
use crate::rules::boligstoette::Boligstoette;
use crate::rules::kontanthjaelp::Kontanthjaelp;
use crate::geo::geo_aggregator::{build_geo_entries, compute_kommune_populations};
use crate::scenario::diff::{compute_diff, DiffResult};
use crate::scenario::incremental::{compute_dirty_set, incremental_evaluate};
use crate::scenario::param_mapping::{ParamId, ParamRuleMapping};
use crate::scenario::scenario::Scenario;
use crate::types::{Beskaeftigelsesstatus, Husstandstype};

const SEED: u64 = 42;
const COUNT: usize = 100_000;
const RULE_IDS: [RuleId; 3] = [RuleId::Kontanthjaelp, RuleId::Boligstoette, RuleId::BoerneYdelse];

// ── JSON response types (public bridge surface) ──

#[derive(Serialize)]
pub struct InitResponse {
    pub count: usize,
    pub baseline: Vec<RuleStatsJson>,
}

#[derive(Serialize)]
pub struct RuleStatsJson {
    pub rule: String,
    pub total: f64,
    pub eligible: usize,
    pub mean: f64,
}

#[derive(Serialize)]
pub struct ScenarioResponse {
    pub per_rule: Vec<RuleDiffJson>,
    pub per_kommune: Vec<KommuneDiffJson>,
    pub top_affected: Vec<AffectedBorgerJson>,
    pub total_affected: usize,
}

#[derive(Serialize)]
pub struct RuleDiffJson {
    pub rule: String,
    pub total_delta: f64,
    pub gained: usize,
    pub lost: usize,
}

#[derive(Serialize)]
pub struct KommuneDiffJson {
    pub kommune_id: u8,
    pub total_delta: f64,
    pub affected_count: usize,
}

#[derive(Serialize)]
pub struct AffectedBorgerJson {
    pub borger_id: u32,
    pub total_delta: f64,
}

#[derive(Serialize)]
pub struct CaseDetailJson {
    pub borger_id: u32,
    pub alder: u8,
    pub kommune_id: u8,
    pub husstandstype: String,
    pub beskaeftigelse: String,
    pub indkomst: f64,
    pub husleje: f64,
    pub boligareal: u16,
    pub antal_boern: u8,
    pub rules: Vec<CaseRuleJson>,
}

#[derive(Serialize)]
pub struct CaseRuleJson {
    pub rule: String,
    pub amount: f64,
    pub eligible: bool,
}

#[derive(Serialize)]
pub struct KommuneGeoJson {
    pub kommune_id: u8,
    pub population: usize,
    pub total_delta: f64,
    pub per_capita_delta: f64,
    pub affected_count: usize,
}

#[derive(Serialize)]
pub struct GeoResponse {
    pub kommuner: Vec<KommuneGeoJson>,
}

#[derive(Serialize)]
pub struct FilteredStatsResponse {
    pub kommune_id: Option<u8>,
    pub count: usize,
    pub rules: Vec<RuleStatsJson>,
}

// ── Engine (holds all state) ──

pub struct Engine {
    store: BorgerStore,
    rules: Vec<Box<dyn Rule>>,
    graph: DependencyGraph,
    params: RuleParams,
    mapping: ParamRuleMapping,
    baseline: BatchResult,
    kommune_populations: [usize; 256],
    last_scenario_result: Option<BatchResult>,
    last_diff: Option<DiffResult>,
}

impl Engine {
    pub fn new() -> Self {
        let store = borger_generator::generate(SEED, COUNT);
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(Kontanthjaelp),
            Box::new(Boligstoette),
            Box::new(BoerneYdelse),
        ];
        let rule_refs: Vec<&dyn Rule> = rules.iter().map(|r| r.as_ref()).collect();
        let graph = DependencyGraph::build(&rule_refs).unwrap();
        let params = RuleParams::default();
        let mapping = ParamRuleMapping::new();
        let baseline = batch_evaluate(&store, &rules, &graph, &params);
        let kommune_populations = compute_kommune_populations(&store);

        Self {
            store,
            rules,
            graph,
            params,
            mapping,
            baseline,
            kommune_populations,
            last_scenario_result: None,
            last_diff: None,
        }
    }

    pub fn init(&self) -> String {
        let response = InitResponse {
            count: self.baseline.count,
            baseline: self.baseline.per_rule.iter().map(|r| RuleStatsJson {
                rule: rule_name(r.rule_id).to_string(),
                total: r.total_amount,
                eligible: r.eligible_count,
                mean: r.mean_amount,
            }).collect(),
        };
        serde_json::to_string(&response).unwrap()
    }

    pub fn get_baseline_stats(&self) -> String {
        let stats: Vec<RuleStatsJson> = self.baseline.per_rule.iter().map(|r| RuleStatsJson {
            rule: rule_name(r.rule_id).to_string(),
            total: r.total_amount,
            eligible: r.eligible_count,
            mean: r.mean_amount,
        }).collect();
        serde_json::to_string(&stats).unwrap()
    }

    pub fn apply_scenario(&mut self, param_id: u8, value: f64) -> String {
        let pid = parse_param_id(param_id).expect("invalid param_id");
        let scenario = Scenario::new(&self.params, pid, value);
        let dirty = compute_dirty_set(&scenario.overrides, &self.mapping, &self.graph, &self.rules);
        let scenario_result = incremental_evaluate(
            &self.store, &self.rules, &self.graph, &self.baseline, &scenario.params, &dirty,
        );
        let diff = compute_diff(&self.store, &self.baseline, &scenario_result, 10);

        let response = ScenarioResponse {
            per_rule: diff.per_rule.iter().map(|d| RuleDiffJson {
                rule: rule_name(d.rule_id).to_string(),
                total_delta: d.total_delta,
                gained: d.gained_eligibility,
                lost: d.lost_eligibility,
            }).collect(),
            per_kommune: diff.per_kommune.iter().map(|s| KommuneDiffJson {
                kommune_id: s.kommune_id,
                total_delta: s.total_delta,
                affected_count: s.affected_count,
            }).collect(),
            top_affected: diff.top_affected.iter().map(|a| AffectedBorgerJson {
                borger_id: a.borger_id,
                total_delta: a.total_delta,
            }).collect(),
            total_affected: diff.total_affected,
        };

        self.last_scenario_result = Some(scenario_result);
        self.last_diff = Some(diff);

        serde_json::to_string(&response).unwrap()
    }

    pub fn get_top_affected(&self, n: usize) -> String {
        let diff = self.last_diff.as_ref().expect("call apply_scenario first");
        let top: Vec<AffectedBorgerJson> = diff.top_affected.iter().take(n).map(|a| {
            AffectedBorgerJson {
                borger_id: a.borger_id,
                total_delta: a.total_delta,
            }
        }).collect();
        serde_json::to_string(&top).unwrap()
    }

    pub fn get_case_detail(&self, borger_id: u32) -> String {
        let idx = self.store.find_by_id(borger_id)
            .unwrap_or_else(|| panic!("borger_id {} not found", borger_id));
        let view = self.store.view(idx);
        let result = &self.baseline.borger_results[idx];

        let response = CaseDetailJson {
            borger_id,
            alder: view.alder,
            kommune_id: view.kommune_id,
            husstandstype: husstandstype_name(view.husstandstype).to_string(),
            beskaeftigelse: beskaeftigelse_name(view.beskaeftigelsesstatus).to_string(),
            indkomst: view.bruttoindkomst,
            husleje: view.husleje,
            boligareal: view.boligareal,
            antal_boern: view.antal_boern,
            rules: RULE_IDS.iter().map(|&rid| CaseRuleJson {
                rule: rule_name(rid).to_string(),
                amount: result.amount(rid),
                eligible: result.is_eligible(rid),
            }).collect(),
        };
        serde_json::to_string(&response).unwrap()
    }

    pub fn get_geo_data(&self) -> String {
        let diff = self.last_diff.as_ref().expect("call apply_scenario first");
        let entries = build_geo_entries(&self.kommune_populations, diff);
        let response = GeoResponse {
            kommuner: entries.iter().map(|e| KommuneGeoJson {
                kommune_id: e.kommune_id,
                population: e.population,
                total_delta: e.total_delta,
                per_capita_delta: e.per_capita_delta,
                affected_count: e.affected_count,
            }).collect(),
        };
        serde_json::to_string(&response).unwrap()
    }

    pub fn get_filtered_stats(&self, kommune_id: Option<u8>) -> String {
        let active_result = self.last_scenario_result.as_ref().unwrap_or(&self.baseline);
        let mut totals = [0.0_f64; 3];
        let mut eligible = [0_usize; 3];
        let mut borger_count = 0_usize;

        for i in 0..active_result.count {
            if let Some(kid) = kommune_id {
                if self.store.kommune_id[i] != kid {
                    continue;
                }
            }
            borger_count += 1;
            let r = &active_result.borger_results[i];
            for (idx, &rid) in RULE_IDS.iter().enumerate() {
                totals[idx] += r.amount(rid);
                if r.is_eligible(rid) {
                    eligible[idx] += 1;
                }
            }
        }

        let rules: Vec<RuleStatsJson> = RULE_IDS.iter().enumerate().map(|(idx, &rid)| {
            RuleStatsJson {
                rule: rule_name(rid).to_string(),
                total: totals[idx],
                eligible: eligible[idx],
                mean: if eligible[idx] > 0 { totals[idx] / eligible[idx] as f64 } else { 0.0 },
            }
        }).collect();

        let response = FilteredStatsResponse {
            kommune_id,
            count: borger_count,
            rules,
        };
        serde_json::to_string(&response).unwrap()
    }
}

// ── Helpers ──

fn rule_name(id: RuleId) -> &'static str {
    match id {
        RuleId::Kontanthjaelp => "kontanthjaelp",
        RuleId::Boligstoette => "boligstoette",
        RuleId::BoerneYdelse => "boerneydelse",
    }
}

fn husstandstype_name(h: Husstandstype) -> &'static str {
    match h {
        Husstandstype::Enlig => "enlig",
        Husstandstype::ParUdenBoern => "par_uden_boern",
        Husstandstype::ParMedBoern => "par_med_boern",
        Husstandstype::EnligForsoerger => "enlig_forsoerger",
    }
}

fn beskaeftigelse_name(b: Beskaeftigelsesstatus) -> &'static str {
    match b {
        Beskaeftigelsesstatus::Fuldtid => "fuldtid",
        Beskaeftigelsesstatus::Deltid => "deltid",
        Beskaeftigelsesstatus::Ledig => "ledig",
        Beskaeftigelsesstatus::Aktivitetsparat => "aktivitetsparat",
        Beskaeftigelsesstatus::Sygemeldt => "sygemeldt",
    }
}

fn parse_param_id(id: u8) -> Option<ParamId> {
    match id {
        0 => Some(ParamId::KontanthjaelpBasis),
        1 => Some(ParamId::Forsoergertillaeg),
        2 => Some(ParamId::BoligstoetteGraense),
        3 => Some(ParamId::BoerneYdelseAftrapning),
        _ => None,
    }
}

// ── WASM Bridge (thin wrappers) ──

thread_local! {
    static WASM_ENGINE: RefCell<Option<Engine>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn wasm_init() -> String {
    WASM_ENGINE.with(|e| {
        let engine = Engine::new();
        let result = engine.init();
        *e.borrow_mut() = Some(engine);
        result
    })
}

#[wasm_bindgen]
pub fn wasm_get_baseline_stats() -> String {
    WASM_ENGINE.with(|e| {
        e.borrow().as_ref().expect("call wasm_init first").get_baseline_stats()
    })
}

#[wasm_bindgen]
pub fn wasm_apply_scenario(param_id: u8, value: f64) -> String {
    WASM_ENGINE.with(|e| {
        e.borrow_mut().as_mut().expect("call wasm_init first").apply_scenario(param_id, value)
    })
}

#[wasm_bindgen]
pub fn wasm_get_top_affected(n: usize) -> String {
    WASM_ENGINE.with(|e| {
        e.borrow().as_ref().expect("call wasm_init first").get_top_affected(n)
    })
}

#[wasm_bindgen]
pub fn wasm_get_case_detail(borger_id: u32) -> String {
    WASM_ENGINE.with(|e| {
        e.borrow().as_ref().expect("call wasm_init first").get_case_detail(borger_id)
    })
}

#[wasm_bindgen]
pub fn wasm_get_geo_data() -> String {
    WASM_ENGINE.with(|e| {
        e.borrow().as_ref().expect("call wasm_init first").get_geo_data()
    })
}

#[wasm_bindgen]
pub fn wasm_get_filtered_stats(kommune_id: i16) -> String {
    let kid = if kommune_id < 0 { None } else { Some(kommune_id as u8) };
    WASM_ENGINE.with(|e| {
        e.borrow().as_ref().expect("call wasm_init first").get_filtered_stats(kid)
    })
}
