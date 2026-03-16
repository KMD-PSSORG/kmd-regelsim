use std::collections::HashMap;
use crate::rule_engine::RuleId;

/// Identifies a slider parameter that can be changed in a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamId {
    KontanthjaelpBasis,
    Forsoergertillaeg,
    BoligstoetteGraense,
    BoerneYdelseAftrapning,
}

/// Explicit mapping from parameter to the root rules it directly affects.
/// Dirty propagation through the dependency graph handles downstream rules.
#[derive(Debug)]
pub struct ParamRuleMapping {
    mapping: HashMap<ParamId, Vec<RuleId>>,
}

impl ParamRuleMapping {
    pub fn new() -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(ParamId::KontanthjaelpBasis, vec![RuleId::Kontanthjaelp]);
        mapping.insert(ParamId::Forsoergertillaeg, vec![RuleId::Kontanthjaelp]);
        mapping.insert(ParamId::BoligstoetteGraense, vec![RuleId::Boligstoette]);
        mapping.insert(ParamId::BoerneYdelseAftrapning, vec![RuleId::BoerneYdelse]);
        Self { mapping }
    }

    /// Returns the root rules directly affected by a parameter change.
    pub fn affected_roots(&self, param_id: ParamId) -> &[RuleId] {
        self.mapping.get(&param_id).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

impl Default for ParamRuleMapping {
    fn default() -> Self {
        Self::new()
    }
}
