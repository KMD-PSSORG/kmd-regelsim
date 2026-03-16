use crate::batch::types::{BatchResult, TopNEntry};
use crate::borger_store::BorgerStore;
use crate::rule_engine::RuleId;

/// Return top-N borgere by amount for a given rule.
/// Uses partial sort via select_nth_unstable for O(n) average.
pub fn top_n(
    store: &BorgerStore,
    result: &BatchResult,
    rule_id: RuleId,
    n: usize,
) -> Vec<TopNEntry> {
    let n = n.min(result.count);

    let mut indices: Vec<usize> = (0..result.count).collect();

    indices.select_nth_unstable_by(n.saturating_sub(1), |&a, &b| {
        let amt_a = result.borger_results[a].amount(rule_id);
        let amt_b = result.borger_results[b].amount(rule_id);
        amt_b.partial_cmp(&amt_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut top: Vec<TopNEntry> = indices[..n]
        .iter()
        .map(|&i| TopNEntry {
            borger_index: i,
            borger_id: store.borger_id[i],
            rule_id,
            amount: result.borger_results[i].amount(rule_id),
        })
        .collect();

    top.sort_unstable_by(|a, b| {
        b.amount.partial_cmp(&a.amount).unwrap_or(std::cmp::Ordering::Equal)
    });

    top
}
