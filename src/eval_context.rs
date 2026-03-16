use std::collections::HashMap;
use crate::rule_engine::{RuleId, RuleResult};

/// Accumulates rule results during evaluation so downstream rules
/// can reference upstream results (e.g. boligstøtte reads kontanthjælp).
#[derive(Debug, Default)]
pub struct EvalContext {
    results: HashMap<RuleId, RuleResult>,
}

impl EvalContext {
    pub fn new() -> Self {
        Self { results: HashMap::new() }
    }

    pub fn insert(&mut self, result: RuleResult) {
        self.results.insert(result.rule_id, result);
    }

    pub fn get(&self, rule_id: &RuleId) -> Option<&RuleResult> {
        self.results.get(rule_id)
    }

    pub fn results(&self) -> &HashMap<RuleId, RuleResult> {
        &self.results
    }

    pub fn clear(&mut self) {
        self.results.clear();
    }
}
