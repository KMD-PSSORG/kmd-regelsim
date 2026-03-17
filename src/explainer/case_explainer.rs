use serde::Serialize;
use crate::borger_store::BorgerStore;
use crate::batch::types::BatchResult;
use crate::dependency_graph::DependencyGraph;
use crate::eval_context::EvalContext;
use crate::explainer::threshold_detector::detect_thresholds;
use crate::rule_engine::{Rule, RuleId, RuleParams, RuleResult};
use crate::types::{Beskaeftigelsesstatus, Husstandstype};

const RULE_IDS: [RuleId; 3] = [RuleId::Kontanthjaelp, RuleId::Boligstoette, RuleId::BoerneYdelse];

#[derive(Debug, Serialize)]
pub struct CaseExplanation {
    pub borger_id: u32,
    pub alder: u8,
    pub kommune_id: u8,
    pub husstandstype: String,
    pub beskaeftigelse: String,
    pub indkomst: f64,
    pub husleje: f64,
    pub boligareal: u16,
    pub antal_boern: u8,
    pub rules: Vec<RuleExplanation>,
}

#[derive(Debug, Serialize)]
pub struct RuleExplanation {
    pub rule: String,
    pub baseline_amount: f64,
    pub scenario_amount: f64,
    pub eligible: bool,
    pub delta: f64,
    pub explanation: String,
    pub threshold_warnings: Vec<String>,
}

/// Generate a full case explanation for a single borger.
pub fn explain_case(
    store: &BorgerStore,
    rules: &[Box<dyn Rule>],
    graph: &DependencyGraph,
    baseline_params: &RuleParams,
    _scenario_params: Option<&RuleParams>,
    baseline_result: &BatchResult,
    scenario_result: Option<&BatchResult>,
    borger_id: u32,
    threshold_pct: f64,
) -> Result<CaseExplanation, String> {
    let idx = store.find_by_id(borger_id)
        .ok_or_else(|| format!("borger_id {} not found", borger_id))?;
    let view = store.view(idx);

    let base_compact = &baseline_result.borger_results[idx];
    let scen_compact = scenario_result.map(|sr| &sr.borger_results[idx]);

    let mut ctx = EvalContext::new();
    let order = graph.evaluation_order();
    for &rule_id in order {
        let rule = rules.iter().find(|r| r.id() == rule_id).unwrap();
        let result = rule.evaluate(&view, &ctx, baseline_params);
        ctx.insert(result);
    }

    let rule_explanations: Vec<RuleExplanation> = RULE_IDS
        .iter()
        .map(|&rid| {
            let baseline_amount = base_compact.amount(rid);
            let scenario_amount = scen_compact.map(|s| s.amount(rid)).unwrap_or(baseline_amount);
            let eligible = base_compact.is_eligible(rid);
            let delta = scenario_amount - baseline_amount;

            let result = ctx.get(&rid).unwrap();
            let warnings = detect_thresholds(&view, rid, result, baseline_params, threshold_pct);
            let explanation = generate_explanation(&view, rid, result, baseline_params);

            RuleExplanation {
                rule: rule_name(rid).to_string(),
                baseline_amount,
                scenario_amount,
                eligible,
                delta,
                explanation,
                threshold_warnings: warnings,
            }
        })
        .collect();

    Ok(CaseExplanation {
        borger_id,
        alder: view.alder,
        kommune_id: view.kommune_id,
        husstandstype: husstandstype_name(view.husstandstype).to_string(),
        beskaeftigelse: beskaeftigelse_name(view.beskaeftigelsesstatus).to_string(),
        indkomst: view.bruttoindkomst,
        husleje: view.husleje,
        boligareal: view.boligareal,
        antal_boern: view.antal_boern,
        rules: rule_explanations,
    })
}

fn generate_explanation(
    view: &crate::borger_view::BorgerView,
    rule_id: RuleId,
    result: &RuleResult,
    params: &RuleParams,
) -> String {
    match rule_id {
        RuleId::Kontanthjaelp => explain_kontanthjaelp(view, result, params),
        RuleId::Boligstoette => explain_boligstoette(view, result, params),
        RuleId::BoerneYdelse => explain_boerneydelse(view, result, params),
    }
}

fn explain_kontanthjaelp(
    view: &crate::borger_view::BorgerView,
    result: &RuleResult,
    params: &RuleParams,
) -> String {
    if !result.eligible {
        return format!(
            "Ikke berettiget: besk\u{00e6}ftigelsesstatus er {}.",
            beskaeftigelse_name(view.beskaeftigelsesstatus)
        );
    }

    let is_single = matches!(
        view.husstandstype,
        Husstandstype::Enlig | Husstandstype::EnligForsoerger
    );
    let status = if is_single { "enlig" } else { "par" };
    let age_group = if view.alder >= 30 { "over 30" } else { "under 30" };
    let basis = if is_single { params.kontanthjaelp_basis_enlig } else { params.kontanthjaelp_basis_par };

    let mut text = format!(
        "{}, {}, basissats {:.0} kr",
        capitalize(status), age_group, basis
    );

    if view.antal_boern > 0 {
        let tillaeg = view.antal_boern as f64 * params.forsoergertillaeg_per_barn;
        text.push_str(&format!(
            ". Fors\u{00f8}rgertill\u{00e6}g: {} b\u{00f8}rn \u{2192} +{:.0} kr",
            view.antal_boern, tillaeg
        ));
    }

    text.push_str(&format!(". I alt: {:.0} kr/md.", result.amount));
    text
}

fn explain_boligstoette(
    view: &crate::borger_view::BorgerView,
    result: &RuleResult,
    _params: &RuleParams,
) -> String {
    if !result.eligible {
        return format!(
            "Ikke berettiget: indkomst {:.0} kr overstiger gr\u{00e6}nsen.",
            view.bruttoindkomst
        );
    }

    format!(
        "Husleje {:.0} kr, boligareal {} m\u{00b2}. Indkomst inkl. kontanthj\u{00e6}lp. St\u{00f8}tte: {:.0} kr/md.",
        view.husleje, view.boligareal, result.amount
    )
}

fn explain_boerneydelse(
    view: &crate::borger_view::BorgerView,
    result: &RuleResult,
    params: &RuleParams,
) -> String {
    if view.antal_boern == 0 {
        return "Ingen b\u{00f8}rn.".to_string();
    }

    let mut parts: Vec<String> = Vec::new();
    let mut counts = [0u8; 4]; // 0-2, 3-6, 7-14, 15-17
    for &age in view.boern_aldre {
        match age {
            0..=2 => counts[0] += 1,
            3..=6 => counts[1] += 1,
            7..=14 => counts[2] += 1,
            15..=17 => counts[3] += 1,
            _ => {}
        }
    }

    let labels = [
        ("0-2 \u{00e5}r", params.boerneydelse_0_2),
        ("3-6 \u{00e5}r", params.boerneydelse_3_6),
        ("7-14 \u{00e5}r", params.boerneydelse_7_14),
        ("15-17 \u{00e5}r", params.boerneydelse_15_17),
    ];

    for (i, &count) in counts.iter().enumerate() {
        if count > 0 {
            parts.push(format!("{} barn {} ({:.0} kr)", count, labels[i].0, labels[i].1));
        }
    }

    let mut text = format!("{} b\u{00f8}rn: {}.", view.antal_boern, parts.join(", "));

    if borger_over_aftrapning(view, params) {
        text.push_str(&format!(
            " Aftrapning: indkomst over {:.0} kr.",
            params.boerneydelse_aftrapning_grænse
        ));
    }

    text.push_str(&format!(" I alt: {:.0} kr/md.", result.amount));
    text
}

fn borger_over_aftrapning(
    view: &crate::borger_view::BorgerView,
    params: &RuleParams,
) -> bool {
    view.bruttoindkomst > params.boerneydelse_aftrapning_grænse
}

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

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
