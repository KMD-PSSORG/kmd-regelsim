---
ward: 1
revision: null
name: "Kolonnebaseret Datastruktur + Syntetisk Data"
epic: "Rust Engine Foundation"
status: "complete"
dependencies: []
layer: "rust"
estimated_tests: 8
created: "2026-03-16"
completed: "2026-03-16"
---
# Ward 001: Kolonnebaseret Datastruktur + Syntetisk Data

## Scope
Fundamentet. Borgerprofiler i struct-of-arrays layout. Data genereres ved init. Hvert borgerrecord har et stabilt borger_id.

## Inputs
Ingen — dette er den første ward.

## Outputs
- `BorgerStore` struct med kolonnebaseret data for 100K borgere
- `BorgerView` zero-copy reference til én borgers data
- Deterministisk data-generator med seedet PRNG
- Realistiske fordelinger (alderspyramide, log-normal indkomst, Poisson børn)
- Stabil `borger_id: u32` per borger

## Specification

### Datamodel
| Felt | Type | Fordeling |
|------|------|-----------|
| borger_id | u32 | Monotont stigende, stabil identifikator |
| alder | u8 | Danmarks Statistik alderspyramide |
| husstandstype | enum | enlig, par_uden_børn, par_med_børn, enlig_forsørger |
| bruttoindkomst | f64 | Log-normalfordelt, median ~310K |
| husleje | f64 | Normalfordelt, regionsjusteret |
| boligareal | u16 | Korreleret med husstandstype |
| antal_børn | u8 | Poissonfordelt, betinget af husstandstype |
| børn_aldre | Vec<u8> | Uniform 0-17 |
| beskæftigelsesstatus | enum | fuldtid, deltid, ledig, aktivitetsparat, sygemeldt |
| kommune_id | u8 | Vægtet efter befolkningstal (98 kommuner) |

### Structs
- `BorgerStore` — struct-of-arrays med `Vec<T>` per felt
- `BorgerGenerator` — seedet PRNG, realistiske fordelinger
- `BorgerView` — zero-copy view over én borger (borger_id → index → references ind i kolonner)

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_create_store_100k | Kan oprette en BorgerStore med 100K entries |
| 2 | test_column_types | Kolonner er korrekt typede (u8, f64, bool, enum) |
| 3 | test_single_borger_view | Kan hente en enkelt borgers fulde profil via borger_id |
| 4 | test_column_iteration | Kan iterere over en kolonne isoleret (cache-friendly access) |
| 5 | test_memory_footprint | Memory footprint under 50MB for 100K borgere |
| 6 | test_realistic_distributions | Syntetisk data har realistiske fordelinger (gennemsnitsindkomst ~310K, aldersfordeling matcher pyramide) |
| 7 | test_deterministic_seed | Deterministisk: samme seed → identiske records for same build target |
| 8 | test_borger_id_stable_unique | borger_id er stabil og unik |

## Must NOT
- Bruge heap-allokering per borger (ingen `Vec<Borger>`)
- Hardcode data — alt genereres fra seed
- Inkludere regellogik (det er Ward 2)

## Must DO
- Struct-of-arrays layout (ikke array-of-structs)
- Deterministisk generering med eksplicit seed
- Under 50MB memory for 100K borgere
- Realistiske fordelinger baseret på danske statistikker
- Stabil borger_id (u32, monotont stigende)

## Verification
`cargo test` — alle 8 tests passerer. Memory-test verificerer footprint. Seed-test verificerer determinisme. borger_id er unik.
