use crate::batch::types::{BatchResult, CompactBorgerResult, RuleAggregation};
use crate::borger_store::BorgerStore;
use crate::dependency_graph::DependencyGraph;
use crate::eval_context::EvalContext;
use crate::rule_engine::{Rule, RuleId, RuleParams};

const RULE_IDS: [RuleId; 3] = [RuleId::Kontanthjaelp, RuleId::Boligstoette, RuleId::BoerneYdelse];

pub fn batch_evaluate(
    store: &BorgerStore,
    rules: &[Box<dyn Rule>],
    graph: &DependencyGraph,
    params: &RuleParams,
) -> BatchResult {
    let count = store.len();
    let mut borger_results = Vec::with_capacity(count);

    let mut totals = [0.0_f64; 3];
    let mut eligible_counts = [0_usize; 3];

    let order = graph.evaluation_order();

    let mut ctx = EvalContext::new();

    for i in 0..count {
        let borger = store.view(i);
        ctx.clear();

        for &rule_id in order {
            let rule = rules.iter().find(|r| r.id() == rule_id).unwrap();
            let result = rule.evaluate(&borger, &ctx, params);
            ctx.insert(result);
        }

        let mut amounts = [0.0_f64; 3];
        let mut eligible = [false; 3];

        for (idx, &rid) in RULE_IDS.iter().enumerate() {
            if let Some(r) = ctx.get(&rid) {
                amounts[idx] = r.amount;
                eligible[idx] = r.eligible;
                totals[idx] += r.amount;
                if r.eligible {
                    eligible_counts[idx] += 1;
                }
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
