use crate::borger_view::BorgerView;
use crate::eval_context::EvalContext;
use crate::rule_engine::{Rule, RuleId, RuleParams, RuleResult};

pub struct BoerneYdelse;

impl Rule for BoerneYdelse {
    fn id(&self) -> RuleId {
        RuleId::BoerneYdelse
    }

    fn dependencies(&self) -> &[RuleId] {
        &[]
    }

    fn evaluate(&self, borger: &BorgerView, _ctx: &EvalContext, params: &RuleParams) -> RuleResult {
        if borger.antal_boern == 0 {
            return RuleResult {
                rule_id: RuleId::BoerneYdelse,
                amount: 0.0,
                eligible: false,
            };
        }

        let mut total: f64 = 0.0;
        for &age in borger.boern_aldre {
            total += match age {
                0..=2 => params.boerneydelse_0_2,
                3..=6 => params.boerneydelse_3_6,
                7..=14 => params.boerneydelse_7_14,
                15..=17 => params.boerneydelse_15_17,
                _ => 0.0,
            };
        }

        if borger.bruttoindkomst > params.boerneydelse_aftrapning_grænse {
            let excess = borger.bruttoindkomst - params.boerneydelse_aftrapning_grænse;
            let reduction = excess * params.boerneydelse_aftrapning_procent / 12.0;
            total = (total - reduction).max(0.0);
        }

        RuleResult {
            rule_id: RuleId::BoerneYdelse,
            amount: total,
            eligible: total > 0.0,
        }
    }
}
