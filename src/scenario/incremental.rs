use std::collections::HashSet;
use crate::batch::types::{BatchResult, CompactBorgerResult, RuleAggregation};
use crate::borger_store::BorgerStore;
use crate::dependency_graph::DependencyGraph;
use crate::eval_context::EvalContext;
use crate::rule_engine::{Rule, RuleId, RuleParams, RuleResult};
use crate::scenario::param_mapping::{ParamId, ParamRuleMapping};

const RULE_IDS: [RuleId; 3] = [RuleId::Kontanthjaelp, RuleId::Boligstoette, RuleId::BoerneYdelse];

fn rule_index(rule_id: RuleId) -> usize {
    match rule_id {
        RuleId::Kontanthjaelp => 0,
        RuleId::Boligstoette => 1,
        RuleId::BoerneYdelse => 2,
    }
}

/// Compute dirty set: root rules affected by param changes + all downstream dependents.
pub fn compute_dirty_set(
    overrides: &[(ParamId, f64)],
    mapping: &ParamRuleMapping,
    _graph: &DependencyGraph,
    all_rules: &[Box<dyn Rule>],
) -> HashSet<RuleId> {
    let mut dirty = HashSet::new();

    for &(param_id, _) in overrides {
        for &root in mapping.affected_roots(param_id) {
            dirty.insert(root);
        }
    }

    propagate_downstream(&mut dirty, all_rules);
    dirty
}

fn propagate_downstream(dirty: &mut HashSet<RuleId>, all_rules: &[Box<dyn Rule>]) {
    let mut changed = true;
    while changed {
        changed = false;
        for rule in all_rules {
            if !dirty.contains(&rule.id()) {
                if rule.dependencies().iter().any(|dep| dirty.contains(dep)) {
                    dirty.insert(rule.id());
                    changed = true;
                }
            }
        }
    }
}

/// Incrementally re-evaluate only dirty rules, reusing baseline results for clean rules.
pub fn incremental_evaluate(
    store: &BorgerStore,
    rules: &[Box<dyn Rule>],
    graph: &DependencyGraph,
    baseline: &BatchResult,
    scenario_params: &RuleParams,
    dirty: &HashSet<RuleId>,
) -> BatchResult {
    let count = store.len();
    let order = graph.evaluation_order();

    // Pre-compute: which rules are dirty, and build a lookup table
    let plan: Vec<(usize, bool)> = order
        .iter()
        .map(|&rid| (rule_index(rid), dirty.contains(&rid)))
        .collect();

    // Pre-resolve rule references by index
    let rule_lookup: Vec<Option<&dyn Rule>> = RULE_IDS
        .iter()
        .map(|rid| rules.iter().find(|r| r.id() == *rid).map(|r| r.as_ref()))
        .collect();

    let any_dirty: Vec<usize> = plan.iter()
        .filter(|(_, is_dirty)| *is_dirty)
        .map(|(idx, _)| *idx)
        .collect();

    let mut borger_results = Vec::with_capacity(count);
    let mut totals = [0.0_f64; 3];
    let mut eligible_counts = [0_usize; 3];
    let mut ctx = EvalContext::new();

    for i in 0..count {
        let base = &baseline.borger_results[i];

        // Fast path: if no rules are dirty, just copy baseline
        if any_dirty.is_empty() {
            for idx in 0..3 {
                totals[idx] += base.amounts[idx];
                if base.eligible[idx] {
                    eligible_counts[idx] += 1;
                }
            }
            borger_results.push(*base);
            continue;
        }

        ctx.clear();

        let borger = store.view(i);
        let mut amounts = base.amounts;
        let mut eligible = base.eligible;

        for &(idx, is_dirty) in &plan {
            if is_dirty {
                let rule = rule_lookup[idx].unwrap();
                let result = rule.evaluate(&borger, &ctx, scenario_params);
                amounts[idx] = result.amount;
                eligible[idx] = result.eligible;
                ctx.insert(result);
            } else {
                ctx.insert(RuleResult {
                    rule_id: RULE_IDS[idx],
                    amount: base.amounts[idx],
                    eligible: base.eligible[idx],
                });
            }
        }

        for idx in 0..3 {
            totals[idx] += amounts[idx];
            if eligible[idx] {
                eligible_counts[idx] += 1;
            }
        }

        borger_results.push(CompactBorgerResult { amounts, eligible });
    }

    let per_rule = RULE_IDS
        .iter()
        .enumerate()
        .map(|(idx, &rid)| {
            let mean = if eligible_counts[idx] > 0 {
                totals[idx] / eligible_counts[idx] as f64
            } else {
                0.0
            };
            RuleAggregation {
                rule_id: rid,
                total_amount: totals[idx],
                eligible_count: eligible_counts[idx],
                mean_amount: mean,
            }
        })
        .collect();

    BatchResult {
        per_rule,
        borger_results,
        count,
    }
}
