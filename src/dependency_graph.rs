use std::collections::{HashMap, VecDeque};
use crate::error::EngineError;
use crate::rule_engine::{Rule, RuleId};

/// Directed acyclic graph of rule dependencies with topological ordering.
#[derive(Debug)]
pub struct DependencyGraph {
    order: Vec<RuleId>,
}

impl DependencyGraph {
    /// Build a dependency graph from a set of rules.
    /// Returns Err(EngineError::CyclicDependency) if cycles are detected.
    pub fn build(rules: &[&dyn Rule]) -> Result<Self, EngineError> {
        let mut adjacency: HashMap<RuleId, Vec<RuleId>> = HashMap::new();
        let mut in_degree: HashMap<RuleId, usize> = HashMap::new();

        for rule in rules {
            let id = rule.id();
            adjacency.entry(id).or_default();
            in_degree.entry(id).or_insert(0);

            for &dep in rule.dependencies() {
                adjacency.entry(dep).or_default().push(id);
                *in_degree.entry(id).or_insert(0) += 1;
            }
        }

        let mut queue: VecDeque<RuleId> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut order = Vec::new();

        while let Some(node) = queue.pop_front() {
            order.push(node);
            if let Some(dependents) = adjacency.get(&node) {
                for &dep in dependents {
                    if let Some(deg) = in_degree.get_mut(&dep) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(dep);
                        }
                    }
                }
            }
        }

        if order.len() != in_degree.len() {
            let remaining: Vec<String> = in_degree
                .iter()
                .filter(|(_, &deg)| deg > 0)
                .map(|(id, _)| format!("{:?}", id))
                .collect();
            return Err(EngineError::CyclicDependency {
                cycle: remaining.join(" → "),
            });
        }

        Ok(Self { order })
    }

    /// Returns rules in evaluation order (dependencies first).
    pub fn evaluation_order(&self) -> &[RuleId] {
        &self.order
    }
}
