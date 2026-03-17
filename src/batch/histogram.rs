use crate::batch::types::BatchResult;
use crate::rule_engine::RuleId;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HistogramBucket {
    pub min: f64,
    pub max: f64,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct HistogramData {
    pub rule: String,
    pub buckets: Vec<HistogramBucket>,
}

pub fn compute_histogram(
    result: &BatchResult,
    rule_id: RuleId,
    bucket_count: usize,
) -> HistogramData {
    let bucket_count = bucket_count.max(2).min(100);

    let mut min_val = f64::MAX;
    let mut max_val = f64::MIN;
    let mut eligible_count = 0_usize;

    for i in 0..result.count {
        let r = &result.borger_results[i];
        if r.is_eligible(rule_id) {
            let amt = r.amount(rule_id);
            if amt < min_val { min_val = amt; }
            if amt > max_val { max_val = amt; }
            eligible_count += 1;
        }
    }

    if eligible_count == 0 || (max_val - min_val).abs() < 0.01 {
        return HistogramData {
            rule: rule_name(rule_id).to_string(),
            buckets: vec![HistogramBucket { min: 0.0, max: 0.0, count: eligible_count }],
        };
    }

    let range = max_val - min_val;
    let step = range / bucket_count as f64;
    let mut counts = vec![0_usize; bucket_count];

    for i in 0..result.count {
        let r = &result.borger_results[i];
        if r.is_eligible(rule_id) {
            let amt = r.amount(rule_id);
            let idx = ((amt - min_val) / step).floor() as usize;
            let idx = idx.min(bucket_count - 1);
            counts[idx] += 1;
        }
    }

    let buckets = (0..bucket_count)
        .map(|i| HistogramBucket {
            min: min_val + i as f64 * step,
            max: min_val + (i + 1) as f64 * step,
            count: counts[i],
        })
        .collect();

    HistogramData {
        rule: rule_name(rule_id).to_string(),
        buckets,
    }
}

fn rule_name(id: RuleId) -> &'static str {
    match id {
        RuleId::Kontanthjaelp => "kontanthjaelp",
        RuleId::Boligstoette => "boligstoette",
        RuleId::BoerneYdelse => "boerneydelse",
    }
}
