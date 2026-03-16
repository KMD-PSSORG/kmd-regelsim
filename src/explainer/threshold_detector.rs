use crate::borger_view::BorgerView;
use crate::rule_engine::{RuleId, RuleParams, RuleResult};
use crate::types::Husstandstype;

/// Detect proximity to rule thresholds/ceilings.
/// Returns human-readable warnings when a value is within `threshold_pct` of a boundary.
pub fn detect_thresholds(
    borger: &BorgerView,
    rule_id: RuleId,
    result: &RuleResult,
    params: &RuleParams,
    threshold_pct: f64,
) -> Vec<String> {
    match rule_id {
        RuleId::Kontanthjaelp => detect_kontanthjaelp(borger, result, params, threshold_pct),
        RuleId::Boligstoette => detect_boligstoette(borger, result, params, threshold_pct),
        RuleId::BoerneYdelse => detect_boerneydelse(borger, result, params, threshold_pct),
    }
}

fn detect_kontanthjaelp(
    borger: &BorgerView,
    result: &RuleResult,
    params: &RuleParams,
    threshold_pct: f64,
) -> Vec<String> {
    let mut warnings = Vec::new();
    if !result.eligible {
        return warnings;
    }

    let is_single = matches!(
        borger.husstandstype,
        Husstandstype::Enlig | Husstandstype::EnligForsoerger
    );
    let loft = if is_single { params.kontanthjaelpsloft_enlig } else { params.kontanthjaelpsloft_par };
    let basis = if is_single { params.kontanthjaelp_basis_enlig } else { params.kontanthjaelp_basis_par };
    let tillaeg = borger.antal_boern as f64 * params.forsoergertillaeg_per_barn;
    let raw = basis + tillaeg;

    if raw > loft * (1.0 - threshold_pct) && raw <= loft {
        let pct = ((loft - raw) / loft * 100.0).abs();
        warnings.push(format!("{:.1}% fra kontanthj\u{00e6}lpsloftet", pct));
    } else if raw > loft {
        warnings.push("Ramt kontanthj\u{00e6}lpsloftet".to_string());
    }

    warnings
}

fn detect_boligstoette(
    borger: &BorgerView,
    result: &RuleResult,
    params: &RuleParams,
    threshold_pct: f64,
) -> Vec<String> {
    let mut warnings = Vec::new();

    let income_basis = borger.bruttoindkomst + result.amount * 12.0;
    let graense = params.boligstoette_grænse;

    if income_basis > graense * (1.0 - threshold_pct) && income_basis <= graense {
        let pct = ((graense - income_basis) / graense * 100.0).abs();
        warnings.push(format!("{:.1}% fra boligst\u{00f8}tte-gr\u{00e6}nsen", pct));
    } else if income_basis > graense * (1.0 - threshold_pct) && !result.eligible {
        warnings.push("Over boligst\u{00f8}tte-gr\u{00e6}nsen".to_string());
    }

    if borger.husleje > params.boligstoette_max_husleje * (1.0 - threshold_pct) {
        let pct = ((params.boligstoette_max_husleje - borger.husleje) / params.boligstoette_max_husleje * 100.0).abs();
        if borger.husleje <= params.boligstoette_max_husleje {
            warnings.push(format!("{:.1}% fra max husleje-gr\u{00e6}nsen", pct));
        }
    }

    warnings
}

fn detect_boerneydelse(
    borger: &BorgerView,
    _result: &RuleResult,
    params: &RuleParams,
    threshold_pct: f64,
) -> Vec<String> {
    let mut warnings = Vec::new();

    if borger.antal_boern == 0 {
        return warnings;
    }

    let graense = params.boerneydelse_aftrapning_grænse;
    let diff = (borger.bruttoindkomst - graense).abs();
    let proximity = diff / graense;

    if proximity < threshold_pct {
        if borger.bruttoindkomst <= graense {
            let pct = (proximity * 100.0).abs();
            warnings.push(format!("{:.1}% fra aftrapningsgr\u{00e6}nsen", pct));
        } else {
            warnings.push("Over aftrapningsgr\u{00e6}nsen".to_string());
        }
    }

    warnings
}
