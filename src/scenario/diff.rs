use crate::batch::types::BatchResult;
use crate::borger_store::BorgerStore;
use crate::rule_engine::RuleId;

const RULE_IDS: [RuleId; 3] = [RuleId::Kontanthjaelp, RuleId::Boligstoette, RuleId::BoerneYdelse];

#[derive(Debug)]
pub struct RuleDiff {
    pub rule_id: RuleId,
    pub total_delta: f64,
    pub gained_eligibility: usize,
    pub lost_eligibility: usize,
}

#[derive(Debug)]
pub struct SegmentDiff {
    pub kommune_id: u8,
    pub total_delta: f64,
    pub affected_count: usize,
}

#[derive(Debug)]
pub struct AffectedBorger {
    pub borger_id: u32,
    pub borger_index: usize,
    pub total_delta: f64,
}

#[derive(Debug)]
pub struct DiffResult {
    pub per_rule: Vec<RuleDiff>,
    pub per_kommune: Vec<SegmentDiff>,
    pub top_affected: Vec<AffectedBorger>,
    pub total_affected: usize,
}

/// Compare baseline vs. scenario results and produce a diff.
pub fn compute_diff(
    store: &BorgerStore,
    baseline: &BatchResult,
    scenario: &BatchResult,
    top_n: usize,
) -> DiffResult {
    let count = baseline.count;

    // Per-rule diffs
    let per_rule: Vec<RuleDiff> = RULE_IDS
        .iter()
        .map(|&rid| {
            let mut total_delta = 0.0;
            let mut gained = 0_usize;
            let mut lost = 0_usize;

            for i in 0..count {
                let base_amt = baseline.borger_results[i].amount(rid);
                let scen_amt = scenario.borger_results[i].amount(rid);
                total_delta += scen_amt - base_amt;

                let base_elig = baseline.borger_results[i].is_eligible(rid);
                let scen_elig = scenario.borger_results[i].is_eligible(rid);
                if !base_elig && scen_elig {
                    gained += 1;
                }
                if base_elig && !scen_elig {
                    lost += 1;
                }
            }

            RuleDiff { rule_id: rid, total_delta, gained_eligibility: gained, lost_eligibility: lost }
        })
        .collect();

    // Per-borger total delta + affected tracking
    let mut borger_deltas: Vec<(usize, f64)> = Vec::with_capacity(count);
    let mut total_affected = 0_usize;

    // Kommune accumulator: [delta, affected_count] indexed by kommune_id
    let mut kommune_delta = [0.0_f64; 256];
    let mut kommune_affected = [0_usize; 256];

    for i in 0..count {
        let base_total = baseline.borger_results[i].total_amount();
        let scen_total = scenario.borger_results[i].total_amount();
        let delta = scen_total - base_total;

        if delta.abs() > 0.001 {
            total_affected += 1;
            let kid = store.kommune_id[i] as usize;
            kommune_delta[kid] += delta;
            kommune_affected[kid] += 1;
        }

        borger_deltas.push((i, delta));
    }

    // Per-kommune segment diffs (only kommuner with affected borgere)
    let per_kommune: Vec<SegmentDiff> = (0..=255_u8)
        .filter(|&kid| kommune_affected[kid as usize] > 0)
        .map(|kid| SegmentDiff {
            kommune_id: kid,
            total_delta: kommune_delta[kid as usize],
            affected_count: kommune_affected[kid as usize],
        })
        .collect();

    // Top-N by |delta| using select_nth_unstable_by for O(n) partial sort
    let top_affected = if borger_deltas.is_empty() || top_n == 0 {
        vec![]
    } else {
        let n = top_n.min(borger_deltas.len());
        borger_deltas.select_nth_unstable_by(n - 1, |a, b| {
            b.1.abs().partial_cmp(&a.1.abs()).unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut top: Vec<_> = borger_deltas[..n].to_vec();
        top.sort_by(|a, b| {
            b.1.abs().partial_cmp(&a.1.abs()).unwrap_or(std::cmp::Ordering::Equal)
        });
        top.iter()
            .map(|&(idx, delta)| AffectedBorger {
                borger_id: store.borger_id[idx],
                borger_index: idx,
                total_delta: delta,
            })
            .collect()
    };

    DiffResult {
        per_rule,
        per_kommune,
        top_affected,
        total_affected,
    }
}
