use crate::rule_engine::RuleParams;
use crate::scenario::param_mapping::ParamId;

/// A scenario: baseline params + one or more parameter overrides.
#[derive(Debug, Clone)]
pub struct Scenario {
    pub params: RuleParams,
    pub overrides: Vec<(ParamId, f64)>,
}

impl Scenario {
    pub fn new(baseline: &RuleParams, param_id: ParamId, value: f64) -> Self {
        let mut params = baseline.clone();
        let overrides = vec![(param_id, value)];
        apply_override(&mut params, param_id, value);
        Self { params, overrides }
    }

    pub fn with_override(mut self, param_id: ParamId, value: f64) -> Self {
        apply_override(&mut self.params, param_id, value);
        self.overrides.push((param_id, value));
        self
    }
}

fn apply_override(params: &mut RuleParams, param_id: ParamId, value: f64) {
    match param_id {
        ParamId::KontanthjaelpBasisEnlig => params.kontanthjaelp_basis_enlig = value,
        ParamId::KontanthjaelpBasisPar => params.kontanthjaelp_basis_par = value,
        ParamId::Forsoergertillaeg => params.forsoergertillaeg_per_barn = value,
        ParamId::BoligstoetteGraense => params.boligstoette_grænse = value,
        ParamId::BoerneYdelseAftrapning => params.boerneydelse_aftrapning_grænse = value,
    }
}
