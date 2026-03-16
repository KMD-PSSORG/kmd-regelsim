use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, LogNormal, Normal};

use crate::borger_store::BorgerStore;
use crate::types::{Beskaeftigelsesstatus, Husstandstype};

/// Danish age distribution weights (0-99), approximating Danmarks Statistik pyramid.
/// Each entry = relative weight for that age.
fn age_weights() -> Vec<f64> {
    let mut w = vec![0.0; 100];
    for age in 0..100 {
        w[age] = match age {
            0..=4 => 1.1,
            5..=9 => 1.2,
            10..=14 => 1.25,
            15..=19 => 1.3,
            20..=24 => 1.35,
            25..=29 => 1.4,
            30..=34 => 1.3,
            35..=39 => 1.25,
            40..=44 => 1.3,
            45..=49 => 1.35,
            50..=54 => 1.5,
            55..=59 => 1.45,
            60..=64 => 1.3,
            65..=69 => 1.15,
            70..=74 => 1.0,
            75..=79 => 0.8,
            80..=84 => 0.55,
            85..=89 => 0.3,
            90..=94 => 0.12,
            95..=99 => 0.03,
            _ => 0.0,
        };
    }
    w
}

/// Cumulative distribution from weights for weighted sampling.
fn build_cumulative(weights: &[f64]) -> Vec<f64> {
    let total: f64 = weights.iter().sum();
    let mut cumulative = Vec::with_capacity(weights.len());
    let mut acc = 0.0;
    for &w in weights {
        acc += w / total;
        cumulative.push(acc);
    }
    if let Some(last) = cumulative.last_mut() {
        *last = 1.0;
    }
    cumulative
}

fn sample_from_cumulative(cumulative: &[f64], rng: &mut ChaCha8Rng) -> usize {
    let r: f64 = rng.gen();
    cumulative.partition_point(|&c| c < r)
}

/// 98 Danish municipalities weighted by approximate population.
/// Index 0 = kommune_id 1 (København), index 97 = kommune_id 98.
fn kommune_weights() -> Vec<f64> {
    let mut w = vec![1.0; 98];
    // Top municipalities by population (approximate relative weights)
    w[0] = 30.0;  // København
    w[1] = 15.0;  // Aarhus
    w[2] = 10.0;  // Odense
    w[3] = 9.0;   // Aalborg
    w[4] = 5.0;   // Esbjerg
    w[5] = 4.5;   // Randers
    w[6] = 4.0;   // Vejle
    w[7] = 4.0;   // Horsens
    w[8] = 3.8;   // Kolding
    w[9] = 3.5;   // Silkeborg
    w[10] = 3.5;  // Herning
    w[11] = 3.2;  // Roskilde
    w[12] = 3.0;  // Næstved
    w[13] = 3.0;  // Frederiksberg
    w[14] = 2.8;  // Viborg
    w[15] = 2.8;  // Holstebro
    w[16] = 2.7;  // Slagelse
    w[17] = 2.5;  // Svendborg
    w[18] = 2.5;  // Hjørring
    w[19] = 2.5;  // Frederikshavn
    // Rest stay at 1.0 (smaller municipalities)
    w
}

fn sample_husstandstype(rng: &mut ChaCha8Rng) -> Husstandstype {
    let r: f64 = rng.gen();
    match r {
        x if x < 0.30 => Husstandstype::Enlig,
        x if x < 0.50 => Husstandstype::ParUdenBoern,
        x if x < 0.82 => Husstandstype::ParMedBoern,
        _ => Husstandstype::EnligForsoerger,
    }
}

fn sample_beskaeftigelse(rng: &mut ChaCha8Rng) -> Beskaeftigelsesstatus {
    let r: f64 = rng.gen();
    match r {
        x if x < 0.55 => Beskaeftigelsesstatus::Fuldtid,
        x if x < 0.70 => Beskaeftigelsesstatus::Deltid,
        x if x < 0.82 => Beskaeftigelsesstatus::Ledig,
        x if x < 0.92 => Beskaeftigelsesstatus::Aktivitetsparat,
        _ => Beskaeftigelsesstatus::Sygemeldt,
    }
}

/// Poisson sampling via inverse CDF (small lambda only).
fn sample_poisson(lambda: f64, rng: &mut ChaCha8Rng) -> u8 {
    let l = (-lambda).exp();
    let mut k: u8 = 0;
    let mut p: f64 = 1.0;
    loop {
        p *= rng.gen::<f64>();
        if p <= l {
            return k;
        }
        k = k.saturating_add(1);
        if k == 10 {
            return k;
        }
    }
}

/// Generates a BorgerStore with `count` synthetic citizens using deterministic PRNG.
/// Same seed always produces identical data.
pub fn generate(seed: u64, count: usize) -> BorgerStore {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let age_cum = build_cumulative(&age_weights());
    let kommune_cum = build_cumulative(&kommune_weights());

    // Log-normal for income: median ~310K → ln(310_000) ≈ 12.644, sigma ~0.5
    let income_dist = LogNormal::new(12.644, 0.5).unwrap();

    // Normal for rent: mean ~6000 kr/month, stddev ~2000
    let rent_dist = Normal::new(6000.0, 2000.0).unwrap();

    let mut borger_id = Vec::with_capacity(count);
    let mut alder = Vec::with_capacity(count);
    let mut husstandstype = Vec::with_capacity(count);
    let mut bruttoindkomst = Vec::with_capacity(count);
    let mut husleje = Vec::with_capacity(count);
    let mut boligareal = Vec::with_capacity(count);
    let mut antal_boern = Vec::with_capacity(count);
    let mut boern_aldre = Vec::with_capacity(count);
    let mut beskaeftigelsesstatus = Vec::with_capacity(count);
    let mut kommune_id = Vec::with_capacity(count);

    for i in 0..count {
        borger_id.push((i + 1) as u32);
        let age = sample_from_cumulative(&age_cum, &mut rng) as u8;
        alder.push(age);

        let ht = sample_husstandstype(&mut rng);
        husstandstype.push(ht);

        let raw_income: f64 = income_dist.sample(&mut rng);
        bruttoindkomst.push(raw_income.round());

        let raw_rent: f64 = rent_dist.sample(&mut rng);
        husleje.push(raw_rent.max(1500.0).round());

        let base_areal: f64 = match ht {
            Husstandstype::Enlig => Normal::new(55.0, 15.0).unwrap().sample(&mut rng),
            Husstandstype::ParUdenBoern => Normal::new(80.0, 20.0).unwrap().sample(&mut rng),
            Husstandstype::ParMedBoern => Normal::new(110.0, 25.0).unwrap().sample(&mut rng),
            Husstandstype::EnligForsoerger => Normal::new(75.0, 18.0).unwrap().sample(&mut rng),
        };
        boligareal.push((base_areal.max(20.0).min(500.0)) as u16);

        let lambda = match ht {
            Husstandstype::Enlig | Husstandstype::ParUdenBoern => 0.0,
            Husstandstype::ParMedBoern => 2.0,
            Husstandstype::EnligForsoerger => 1.5,
        };
        let n_children = if lambda > 0.0 {
            sample_poisson(lambda, &mut rng).max(1)
        } else {
            0
        };
        antal_boern.push(n_children);

        let children_ages: Vec<u8> = (0..n_children)
            .map(|_| rng.gen_range(0..=17))
            .collect();
        boern_aldre.push(children_ages);

        beskaeftigelsesstatus.push(sample_beskaeftigelse(&mut rng));

        let kid = sample_from_cumulative(&kommune_cum, &mut rng) as u8 + 1;
        kommune_id.push(kid);
    }

    BorgerStore {
        borger_id,
        alder,
        husstandstype,
        bruttoindkomst,
        husleje,
        boligareal,
        antal_boern,
        boern_aldre,
        beskaeftigelsesstatus,
        kommune_id,
    }
}
