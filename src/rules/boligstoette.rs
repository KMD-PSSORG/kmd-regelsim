use crate::borger_view::BorgerView;
use crate::eval_context::EvalContext;
use crate::rule_engine::{Rule, RuleId, RuleParams, RuleResult};

pub struct Boligstoette;

impl Rule for Boligstoette {
    fn id(&self) -> RuleId {
        RuleId::Boligstoette
    }

    fn dependencies(&self) -> &[RuleId] {
        &[RuleId::Kontanthjaelp]
    }

    fn evaluate(&self, borger: &BorgerView, ctx: &EvalContext, params: &RuleParams) -> RuleResult {
        let kh_amount = ctx
            .get(&RuleId::Kontanthjaelp)
            .map(|r| r.amount)
            .unwrap_or(0.0);

        let income_basis = borger.bruttoindkomst + kh_amount * 12.0;

        if income_basis > params.boligstoette_grænse {
            return RuleResult {
                rule_id: RuleId::Boligstoette,
                amount: 0.0,
                eligible: false,
            };
        }

        let eligible_rent = borger.husleje.min(params.boligstoette_max_husleje);

        let income_reduction = (income_basis / params.boligstoette_grænse) * eligible_rent * (1.0 - params.boligstoette_procent);

        let amount = (eligible_rent * params.boligstoette_procent - income_reduction).max(0.0);

        RuleResult {
            rule_id: RuleId::Boligstoette,
            amount,
            eligible: amount > 0.0,
        }
    }
}
