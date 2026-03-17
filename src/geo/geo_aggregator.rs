use crate::borger_store::BorgerStore;
use crate::scenario::diff::DiffResult;

pub const KOMMUNE_COUNT: usize = 98;

pub struct KommuneGeoEntry {
    pub kommune_id: u8,
    pub population: usize,
    pub total_delta: f64,
    pub per_capita_delta: f64,
    pub affected_count: usize,
}

/// Count population per kommune (IDs 1..=98).
pub fn compute_kommune_populations(store: &BorgerStore) -> [usize; 256] {
    let mut pops = [0_usize; 256];
    for &kid in &store.kommune_id {
        pops[kid as usize] += 1;
    }
    pops
}

/// Build full geo data for all 98 kommuner, merging population counts with diff data.
pub fn build_geo_entries(
    populations: &[usize; 256],
    diff: &DiffResult,
) -> Vec<KommuneGeoEntry> {
    let mut delta_map = [0.0_f64; 256];
    let mut affected_map = [0_usize; 256];

    for seg in &diff.per_kommune {
        delta_map[seg.kommune_id as usize] = seg.total_delta;
        affected_map[seg.kommune_id as usize] = seg.affected_count;
    }

    (1..=KOMMUNE_COUNT as u8)
        .map(|kid| {
            let pop = populations[kid as usize];
            let delta = delta_map[kid as usize];
            let per_capita = if pop > 0 { delta / pop as f64 } else { 0.0 };
            KommuneGeoEntry {
                kommune_id: kid,
                population: pop,
                total_delta: delta,
                per_capita_delta: per_capita,
                affected_count: affected_map[kid as usize],
            }
        })
        .collect()
}
