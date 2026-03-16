use kmd_engine::borger_generator;
use kmd_engine::types::{Beskaeftigelsesstatus, Husstandstype};

const SEED: u64 = 42;
const COUNT: usize = 100_000;

#[test]
fn test_create_store_100k() {
    let store = borger_generator::generate(SEED, COUNT);
    assert_eq!(store.len(), COUNT);
    assert!(!store.is_empty());
}

#[test]
fn test_column_types() {
    let store = borger_generator::generate(SEED, 1_000);

    for i in 0..store.len() {
        // alder: u8, valid range 0-120
        assert!(store.alder[i] <= 120, "alder out of range: {}", store.alder[i]);

        // husstandstype: valid enum variant (covered by type system, but verify distribution)
        let _ = store.husstandstype[i];

        // bruttoindkomst: f64, non-negative
        assert!(store.bruttoindkomst[i] >= 0.0, "negative indkomst");

        // husleje: f64, non-negative
        assert!(store.husleje[i] >= 0.0, "negative husleje");

        // boligareal: u16, plausible range
        assert!(store.boligareal[i] > 0, "boligareal is zero");
        assert!(store.boligareal[i] <= 500, "boligareal implausibly large: {}", store.boligareal[i]);

        // antal_boern: u8, max realistic
        assert!(store.antal_boern[i] <= 10, "unrealistic antal_boern: {}", store.antal_boern[i]);

        // boern_aldre: Vec<u8>, length matches antal_boern, all 0-17
        assert_eq!(
            store.boern_aldre[i].len(),
            store.antal_boern[i] as usize,
            "boern_aldre length mismatch for borger {}", i
        );
        for &age in &store.boern_aldre[i] {
            assert!(age <= 17, "child age out of range: {}", age);
        }

        // kommune_id: 1-98
        assert!(store.kommune_id[i] >= 1 && store.kommune_id[i] <= 98,
            "kommune_id out of range: {}", store.kommune_id[i]);
    }
}

#[test]
fn test_single_borger_view() {
    let store = borger_generator::generate(SEED, 1_000);

    let view = store.view(0);

    // View fields should match column values at index 0
    assert_eq!(view.borger_id, store.borger_id[0]);
    assert_eq!(view.alder, store.alder[0]);
    assert_eq!(view.husstandstype, store.husstandstype[0]);
    assert_eq!(view.bruttoindkomst, store.bruttoindkomst[0]);
    assert_eq!(view.husleje, store.husleje[0]);
    assert_eq!(view.boligareal, store.boligareal[0]);
    assert_eq!(view.antal_boern, store.antal_boern[0]);
    assert_eq!(view.boern_aldre, store.boern_aldre[0].as_slice());
    assert_eq!(view.beskaeftigelsesstatus, store.beskaeftigelsesstatus[0]);
    assert_eq!(view.kommune_id, store.kommune_id[0]);

    // Also verify a borger in the middle
    let mid = 500;
    let view_mid = store.view(mid);
    assert_eq!(view_mid.alder, store.alder[mid]);
    assert_eq!(view_mid.bruttoindkomst, store.bruttoindkomst[mid]);
    assert_eq!(view_mid.kommune_id, store.kommune_id[mid]);
}

#[test]
fn test_column_iteration() {
    let store = borger_generator::generate(SEED, COUNT);

    // Iterate over alder column in isolation — this is the cache-friendly access pattern
    let sum_alder: u64 = store.alder.iter().map(|&a| a as u64).sum();
    let mean_alder = sum_alder as f64 / COUNT as f64;
    // Danish mean age is roughly 41-42
    assert!(mean_alder > 30.0, "mean alder too low: {}", mean_alder);
    assert!(mean_alder < 55.0, "mean alder too high: {}", mean_alder);

    // Iterate over bruttoindkomst column in isolation
    let sum_indkomst: f64 = store.bruttoindkomst.iter().sum();
    let mean_indkomst = sum_indkomst / COUNT as f64;
    // Target median ~310K, mean will be higher due to log-normal skew
    assert!(mean_indkomst > 200_000.0, "mean indkomst too low: {}", mean_indkomst);
    assert!(mean_indkomst < 600_000.0, "mean indkomst too high: {}", mean_indkomst);
}

#[test]
fn test_memory_footprint() {
    let store = borger_generator::generate(SEED, COUNT);

    let bytes = store.heap_size_bytes();
    let mb = bytes as f64 / (1024.0 * 1024.0);

    assert!(
        mb < 50.0,
        "Memory footprint too large: {:.1} MB (limit: 50 MB)",
        mb
    );

    // Sanity: should be at least a few MB for 100K borgere
    assert!(
        mb > 1.0,
        "Memory footprint suspiciously small: {:.1} MB",
        mb
    );
}

#[test]
fn test_realistic_distributions() {
    let store = borger_generator::generate(SEED, COUNT);

    // --- Age distribution: should roughly follow Danish demographics ---
    let young = store.alder.iter().filter(|&&a| a < 18).count();
    let working = store.alder.iter().filter(|&&a| a >= 18 && a < 65).count();
    let elderly = store.alder.iter().filter(|&&a| a >= 65).count();

    let young_pct = young as f64 / COUNT as f64 * 100.0;
    let working_pct = working as f64 / COUNT as f64 * 100.0;
    let elderly_pct = elderly as f64 / COUNT as f64 * 100.0;

    // Danish demographics: ~20% under 18, ~60% 18-64, ~20% 65+
    assert!(young_pct > 10.0 && young_pct < 30.0,
        "young % out of range: {:.1}%", young_pct);
    assert!(working_pct > 45.0 && working_pct < 75.0,
        "working age % out of range: {:.1}%", working_pct);
    assert!(elderly_pct > 10.0 && elderly_pct < 30.0,
        "elderly % out of range: {:.1}%", elderly_pct);

    // --- Income distribution: log-normal, median ~310K ---
    let mut incomes: Vec<f64> = store.bruttoindkomst.clone();
    incomes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_income = incomes[COUNT / 2];

    assert!(median_income > 250_000.0 && median_income < 400_000.0,
        "median indkomst out of range: {:.0}", median_income);

    // --- Husstandstype distribution: all variants present ---
    let enlig = store.husstandstype.iter().filter(|&&h| h == Husstandstype::Enlig).count();
    let par_med = store.husstandstype.iter().filter(|&&h| h == Husstandstype::ParMedBoern).count();
    let enlig_fors = store.husstandstype.iter().filter(|&&h| h == Husstandstype::EnligForsoerger).count();

    assert!(enlig > 0, "no Enlig in data");
    assert!(par_med > 0, "no ParMedBoern in data");
    assert!(enlig_fors > 0, "no EnligForsoerger in data");

    // Enlig forsørger should be a smaller segment
    assert!((enlig_fors as f64 / COUNT as f64) < 0.25,
        "EnligForsoerger unrealistically common");

    // --- Kommune distribution: all 98 present ---
    let mut kommune_seen = [false; 99]; // index 1..98
    for &kid in &store.kommune_id {
        kommune_seen[kid as usize] = true;
    }
    let unique_kommuner = kommune_seen[1..=98].iter().filter(|&&b| b).count();
    assert_eq!(unique_kommuner, 98, "not all 98 kommuner represented");

    // --- Beskæftigelsesstatus: all variants present ---
    let ledige = store.beskaeftigelsesstatus.iter()
        .filter(|&&b| b == Beskaeftigelsesstatus::Ledig).count();
    assert!(ledige > 0, "no Ledig in data");
    assert!((ledige as f64 / COUNT as f64) < 0.30,
        "unrealistically high unemployment");
}

#[test]
fn test_deterministic_seed() {
    let store_a = borger_generator::generate(SEED, 10_000);
    let store_b = borger_generator::generate(SEED, 10_000);

    // Same seed → identical data
    assert_eq!(store_a.borger_id, store_b.borger_id, "borger_id differs between runs");
    assert_eq!(store_a.alder, store_b.alder, "alder differs between runs");
    assert_eq!(store_a.husstandstype, store_b.husstandstype, "husstandstype differs");
    assert_eq!(store_a.bruttoindkomst, store_b.bruttoindkomst, "bruttoindkomst differs");
    assert_eq!(store_a.husleje, store_b.husleje, "husleje differs");
    assert_eq!(store_a.boligareal, store_b.boligareal, "boligareal differs");
    assert_eq!(store_a.antal_boern, store_b.antal_boern, "antal_boern differs");
    assert_eq!(store_a.boern_aldre, store_b.boern_aldre, "boern_aldre differs");
    assert_eq!(store_a.beskaeftigelsesstatus, store_b.beskaeftigelsesstatus, "beskaeftigelsesstatus differs");
    assert_eq!(store_a.kommune_id, store_b.kommune_id, "kommune_id differs");

    // Different seed → different data
    let store_c = borger_generator::generate(SEED + 1, 10_000);
    assert_ne!(store_a.bruttoindkomst, store_c.bruttoindkomst,
        "different seeds produced identical indkomst — PRNG not seeded correctly");
}

#[test]
fn test_borger_id_stable_unique() {
    let store = borger_generator::generate(SEED, COUNT);

    // borger_id should be monotonically increasing starting at 1
    for i in 0..store.len() {
        assert_eq!(store.borger_id[i], (i + 1) as u32,
            "borger_id not monotonically increasing at index {}", i);
    }

    // All IDs should be unique (implied by monotonic, but verify explicitly)
    let mut seen = std::collections::HashSet::new();
    for &id in &store.borger_id {
        assert!(seen.insert(id), "duplicate borger_id: {}", id);
    }

    // find_by_id should locate the correct index
    let test_id = 42_u32;
    let idx = store.find_by_id(test_id).expect("borger_id 42 not found");
    assert_eq!(store.borger_id[idx], test_id);
    assert_eq!(store.view(idx).borger_id, test_id);

    // Non-existent ID returns None
    assert!(store.find_by_id(0).is_none(), "borger_id 0 should not exist");
    assert!(store.find_by_id(COUNT as u32 + 1).is_none(), "borger_id beyond count should not exist");
}
