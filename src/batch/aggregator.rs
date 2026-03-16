use std::collections::HashMap;
use crate::batch::types::{BatchResult, SegmentAggregation, SegmentKey};
use crate::rule_engine::RuleId;

pub fn aggregate_by_segment(
    result: &BatchResult,
    segment_fn: impl Fn(usize) -> SegmentKey,
    rule_id: RuleId,
) -> Vec<SegmentAggregation> {
    let mut segments: HashMap<SegmentKey, (f64, usize, usize)> = HashMap::new();

    for i in 0..result.count {
        let key = segment_fn(i);
        let cbr = &result.borger_results[i];
        let amount = cbr.amount(rule_id);
        let eligible = cbr.is_eligible(rule_id);

        let entry = segments.entry(key.clone()).or_insert((0.0, 0, 0));
        entry.0 += amount;
        if eligible {
            entry.1 += 1;
        }
        entry.2 += 1;
    }

    segments
        .into_iter()
        .map(|(key, (total, elig, count))| SegmentAggregation {
            key,
            rule_id,
            total_amount: total,
            eligible_count: elig,
            borger_count: count,
        })
        .collect()
}
