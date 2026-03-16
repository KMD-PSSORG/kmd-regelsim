use crate::borger_view::BorgerView;
use crate::eval_context::EvalContext;
use crate::rule_engine::{Rule, RuleId, RuleParams, RuleResult};
use crate::types::{Beskaeftigelsesstatus, Husstandstype};

pub struct Kontanthjaelp;

impl Rule for Kontanthjaelp {
    fn id(&self) -> RuleId {
        RuleId::Kontanthjaelp
    }

    fn dependencies(&self) -> &[RuleId] {
        &[]
    }

    fn evaluate(&self, borger: &BorgerView, _ctx: &EvalContext, params: &RuleParams) -> RuleResult {
        let eligible = matches!(
            borger.beskaeftigelsesstatus,
            Beskaeftigelsesstatus::Ledig | Beskaeftigelsesstatus::Aktivitetsparat
        );

        if !eligible {
            return RuleResult {
                rule_id: RuleId::Kontanthjaelp,
                amount: 0.0,
                eligible: false,
            };
        }

        let is_single = matches!(
            borger.husstandstype,
            Husstandstype::Enlig | Husstandstype::EnligForsoerger
        );

        let basis = if is_single {
            params.kontanthjaelp_basis_enlig
        } else {
            params.kontanthjaelp_basis_par
        };

        let tillaeg = borger.antal_boern as f64 * params.forsoergertillaeg_per_barn;

        let loft = if is_single {
            params.kontanthjaelpsloft_enlig
        } else {
            params.kontanthjaelpsloft_par
        };

        let amount = (basis + tillaeg).min(loft);

        RuleResult {
            rule_id: RuleId::Kontanthjaelp,
            amount,
            eligible: true,
        }
    }
}
