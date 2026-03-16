use crate::borger_view::BorgerView;
use crate::eval_context::EvalContext;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuleResult {
    pub rule_id: RuleId,
    pub amount: f64,
    pub eligible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleId {
    Kontanthjaelp,
    Boligstoette,
    BoerneYdelse,
}

/// Configurable parameters for all rules.
#[derive(Debug, Clone)]
pub struct RuleParams {
    pub kontanthjaelp_basis_enlig: f64,
    pub kontanthjaelp_basis_par: f64,
    pub forsoergertillaeg_per_barn: f64,
    pub kontanthjaelpsloft_enlig: f64,
    pub kontanthjaelpsloft_par: f64,

    pub boligstoette_grænse: f64,
    pub boligstoette_max_husleje: f64,
    pub boligstoette_procent: f64,

    pub boerneydelse_0_2: f64,
    pub boerneydelse_3_6: f64,
    pub boerneydelse_7_14: f64,
    pub boerneydelse_15_17: f64,
    pub boerneydelse_aftrapning_grænse: f64,
    pub boerneydelse_aftrapning_procent: f64,
}

impl Default for RuleParams {
    fn default() -> Self {
        Self {
            kontanthjaelp_basis_enlig: 12_550.0,
            kontanthjaelp_basis_par: 8_710.0,
            forsoergertillaeg_per_barn: 1_710.0,
            kontanthjaelpsloft_enlig: 15_500.0,
            kontanthjaelpsloft_par: 21_000.0,

            boligstoette_grænse: 350_000.0,
            boligstoette_max_husleje: 10_000.0,
            boligstoette_procent: 0.60,

            boerneydelse_0_2: 1_573.0,
            boerneydelse_3_6: 1_246.0,
            boerneydelse_7_14: 980.0,
            boerneydelse_15_17: 327.0,
            boerneydelse_aftrapning_grænse: 828_100.0,
            boerneydelse_aftrapning_procent: 0.02,
        }
    }
}

pub trait Rule {
    fn id(&self) -> RuleId;
    fn dependencies(&self) -> &[RuleId];
    fn evaluate(&self, borger: &BorgerView, ctx: &EvalContext, params: &RuleParams) -> RuleResult;
}
