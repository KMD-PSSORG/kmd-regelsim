# kmd-regelsim v3 — Ward-Driven Development Plan

**Browser-native konsekvenssimulator for regelændringer i dansk offentlig forvaltning**

Built with: Rust/WASM · Vanilla JS render layer · WDD methodology
Author: Dennis Ejby Schmock
Date: March 2026
Version: 3.0 (post dual-round multi-model review — Claude Opus 4.6 + GPT 5.4)

---

## North Star

> **Primary success criterion: a user can change a parameter and understand the effect on totals, segments, geography, and one concrete case in under 100ms, without a backend.**

Alt der ikke bidrager til den sætning, bidrager ikke til demoen.

### Succesmåling (splittet)

| Lag | Mål | Måles i |
|-----|-----|---------|
| **Brugerløfte** | Slider → forståelig opdatering under 100ms (100K) | Ward 11 demo |
| **Engine** | Incremental diff-resultat under 30ms (100K) | Ward 9 benchmark |
| **Bridge** | WASM→JS transfer under 5ms | Ward 5 test |
| **Render** | DOM/SVG update under 16ms | Ward 6 devtools |
| **End-to-end** | Slider → pixels under 100ms (100K), under 200ms (500K) | Ward 9 benchmark |

Hvis et delsystem bliver flaskehals, viser denne tabel præcis hvor problemet bor.

---

## Project Identity

| Key | Value |
|-----|-------|
| Navn | kmd-regelsim |
| Ejer | KMD / Dennis Schmock (bygget i KMD-regi) |
| Ikke relateret til | VectorCore / vgrid / vcore (separat IP, separat firma) |
| Metodik | Ward-Driven Development (WDD) med GS-TDD enforcement |
| Framework | Ingen. Pragmatisk vanilla JS render layer. |
| Domænemodel | Syntetisk og plausibel — ikke juridisk autoritativ |
| Video | Build log optages undervejs |

### Disclaimer

kmd-regelsim er en **teknologidemo med plausibel domænemodel**, ikke et autoritativt regelsystem. Ydelsesberegningerne er forenklede og inspireret af danske regler, men er hverken juridisk korrekte eller administrativt autoritative. Formålet er at demonstrere Rust/WASM-teknologi, ikke at modellere dansk lovgivning.

---

## Arkitekturprincipper

### 1. North Star Test

Enhver feature, enhver ward, enhver beslutning vurderes mod: bidrager dette til at en bruger kan ændre en parameter og forstå effekten på totaler, segmenter, geografi og én konkret case i under 100ms, uden backend? Hvis nej → ikke i v1.

### 2. Rule of Two

Intet enters kmd-engine kernen medmindre mindst 2 views har brug for det.

kmd-regelsim har tre views:

| View | Formål |
|------|--------|
| **Dashboard** | Sliders, stats-panel, histogram, diff-overlay |
| **Case Explainer** | Borger-drilldown med regelkæde og forklaringer |
| **Geo View** | Kommunekort med heatmap |

**Kernen ejer** det mindst 2 views bruger:

| Feature | Dashboard | Explainer | Geo | → Placering |
|---------|:---------:|:---------:|:---:|-------------|
| Batch-evaluering | ✓ | ✓ | ✓ | **Kerne** |
| Aggregering (sum, count, mean) | ✓ | | ✓ | **Kerne** |
| Incremental recompute | ✓ | ✓ | ✓ | **Kerne** |
| Scenarie-diff | ✓ | ✓ | ✓ | **Kerne** |
| Top-N ranking | ✓ | ✓ | | **Kerne** |
| Segment group-by | ✓ | | ✓ | **Kerne** |
| Forklaringskæde (on-demand) | | ✓ | | **Explainer-modul** |
| Histogram-buckets | ✓ | | | **Dashboard-modul** |
| Kommune-aggregering | | | ✓ | **Geo-modul** |
| Threshold/outlier detection | | ✓ | | **Explainer-modul** |

### 3. Rust ejer compute, JS ejer pixels

Rust bestemmer *hvad* der skal vises. JS bestemmer *hvordan* det males. Ingen af dem krydser grænsen. Det er ikke et manifest — det er den pragmatiske opdeling for denne app.

### 4. Plausibel, ikke korrekt

Domænemodellen er troværdig nok til at demoen føles reel, men eksplicit forenklet. Tre regler, realistiske satser, syntetisk data. Ingen forsøg på at modellere dansk lovgivning i fuldstændighed.

### 5. Dependencies

Brug små, velforståede crates når de fjerner boilerplate eller reducerer fejlrisiko. Undgå dependencies der tilføjer abstraktion uden målbar værdi, specielt i hot paths.

| Kategori | Tilladt | Eksempler |
|----------|---------|-----------|
| Serialisering | ✓ | serde, serde_json |
| Fejlhåndtering | ✓ | thiserror |
| Små datastrukturer | ✓ | smallvec, bitvec (hvis profiling viser behov) |
| WASM-binding | ✓ | wasm-bindgen, wasm-pack |
| Benchmarking (dev) | ✓ | criterion |
| Tunge frameworks | ✗ | — |
| Async-økosystemer | ✗ | — |
| Kitchen-sink utils | ✗ | — |

**Arrow:** Ikke i v1. Simpel struct-of-arrays er hurtigere at bygge og lettere at debugge for 3 regler og 100K rækker. Arrow evalueres som substrat hvis projektet vokser til bredere interoperabilitet.

### 6. JSON-reglen

Public WASM bridge returnerer JSON i v1. Internal engine data forbliver typed Rust structs. Ingen JSON i hot internal paths. Aldrig.

### 7. Non-goals for v1

Disse er eksplicit udelukket — ikke fordi de er dårlige idéer, men fordi de vil dræbe v1 hvis de sniger sig ind:

- No general rule DSL
- No plugin system or rule registry
- No persistence layer
- No authentication or backend
- No attempt at legal correctness
- No generic charting framework
- No reusable UI component library
- No config-driven rule loading (regler er Rust structs, ikke YAML)
- No multi-language support beyond da/en labels
- No Arrow, Parquet, or external data format support

---

## Arkitektur

```
┌─────────────────────────────────────────────────────────┐
│                    Browser (ingen server)                │
│                                                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │              JS Render Layer                       │  │
│  │                                                    │  │
│  │  ┌─────────────┐ ┌──────────┐ ┌───────────────┐  │  │
│  │  │  Dashboard   │ │  Case    │ │  Geo View     │  │  │
│  │  │  DOM + SVG   │ │ Explainer│ │  SVG kort     │  │  │
│  │  │  sliders,    │ │ DOM panel│ │  98 kommuner  │  │  │
│  │  │  stats, hist │ │ regelkæde│ │  heatmap      │  │  │
│  │  └──────┬───────┘ └────┬─────┘ └──────┬────────┘  │  │
│  │         │              │              │            │  │
│  │         └──────────────┴──────────────┘            │  │
│  │                        │ events op / data ned      │  │
│  └────────────────────────┼───────────────────────────┘  │
│                           │                              │
│  ┌────────────────────────▼───────────────────────────┐  │
│  │              kmd-engine (Rust → WASM)               │  │
│  │                                                     │  │
│  │  ┌─────────┐ ┌──────────┐ ┌─────────┐ ┌────────┐  │  │
│  │  │  Regel- │ │ Batch    │ │Scenario │ │ WASM   │  │  │
│  │  │  motor  │ │ eval +   │ │ diff +  │ │ API    │  │  │
│  │  │  + dep  │ │ kerne-   │ │ incr.   │ │ bridge │  │  │
│  │  │  graph  │ │ aggregat │ │ recomp  │ │        │  │  │
│  │  └─────────┘ └──────────┘ └─────────┘ └────────┘  │  │
│  │                                                     │  │
│  │  ┌─────────────────────────────────────────────┐   │  │
│  │  │    Borger Store (struct-of-arrays, egen SoA)  │   │  │
│  │  │    100K baseline / 500K stretch               │   │  │
│  │  └─────────────────────────────────────────────┘   │  │
│  └─────────────────────────────────────────────────────┘  │
│                                                           │
└───────────────────────────────────────────────────────────┘
```

---

## Domænemodel: 3 Regler (v1)

| Regel | Nøgleparametre | Afhænger af |
|-------|----------------|-------------|
| **Kontanthjælp** | Basissats (enlig/par), forsørgertillæg, kontanthjælpsloft | — (root) |
| **Boligstøtte** | Husleje, husstandsindkomst, boligareal, grænsebeløb | Kontanthjælp |
| **Børne- og ungeydelse** | Alderstrin (0-2, 3-6, 7-14, 15-17), indkomstaftrapning | — (root) |

Tre regler demonstrerer: dependency graph (boligstøtte → kontanthjælp), segmentering (familier vs. enlige), alderseffekter (børneydelse), slider-interaktion (ændr basissats → downstream-effekt).

**Parameter→regel mapping** (eksplicit, bruges til dirty-marking):

| Slider-parameter | Affected root rules | Dirty propagation |
|------------------|--------------------|--------------------|
| Kontanthjælp basissats | Kontanthjælp | → Boligstøtte (downstream) |
| Forsørgertillæg | Kontanthjælp | → Boligstøtte (downstream) |
| Boligstøtte grænsebeløb | Boligstøtte | (ingen downstream) |
| Børneydelse aftrapningsgrænse | Børneydelse | (ingen downstream) |

**Roadmap-regler** (ikke i v1): Særlig støtte §34, seniorpension, aktivitetstillæg, integrationsydelse.

---

## Syntetisk Data

100K borgerprofiler (500K stretch) med deterministisk seed:

| Felt | Type | Fordeling |
|------|------|-----------|
| borger_id | u32 | Monotont stigende, stabil identifikator |
| alder | u8 | Danmarks Statistik alderspyramide |
| husstandstype | enum | enlig, par_uden_børn, par_med_børn, enlig_forsørger |
| bruttoindkomst | f64 | Log-normalfordelt, median ~310K |
| husleje | f64 | Normalfordelt, regionsjusteret |
| boligareal | u16 | Korreleret med husstandstype |
| antal_børn | u8 | Poissonfordelt, betinget af husstandstype |
| børn_aldre | Vec\<u8\> | Uniform 0-17, simpel liste (ikke pakket i v1) |
| beskæftigelsesstatus | enum | fuldtid, deltid, ledig, aktivitetsparat, sygemeldt |
| kommune_id | u8 | Vægtet efter befolkningstal (98 kommuner) |

Genereret i Rust ved init. Ingen eksterne filer. Samme seed → identiske data.

---

## Ward-Struktur: 12 Wards i 4 Epics

### Epic 1: Rust Engine Foundation (Ward 1–4)

---

#### Ward 1 — Kolonnebaseret Datastruktur + Syntetisk Data

**Formål:** Fundamentet. Borgerprofiler i struct-of-arrays layout. Data genereres ved init.

**Tests (Red):**
- Kan oprette en BorgerStore med 100K entries
- Kolonner er korrekt typede (u8, f64, bool, enum)
- Kan hente en enkelt borgers fulde profil via borger_id
- Kan iterere over en kolonne isoleret (cache-friendly access)
- Memory footprint under 50MB for 100K borgere
- Syntetisk data har realistiske fordelinger (spot-check: gennemsnitsindkomst ~310K, aldersfordeling matcher pyramide)
- Deterministisk: samme seed → identiske records og fordelinger for same build target
- borger_id er stabil og unik

**Implementering (Gold):**
- `borger_store.rs` — struct-of-arrays med Vec\<T\> per felt
- `borger_generator.rs` — seedet PRNG, realistiske fordelinger
- `borger_view.rs` — zero-copy view over en enkelt borger (borger_id → index → references ind i kolonnerne)

**Done:** 100K borgere i hukommelsen med stabile ID'er. Klar til regelmotor.

---

#### Ward 2 — Regelmotor med Dependency Graph

**Formål:** Kernen. Evaluerer regler i korrekt rækkefølge. Kompakte resultater, ingen forklaringer i batch.

**Tests (Red):**
- En regel evalueres for én borger → `RuleResult { rule_id, amount: f64, eligible: bool }`
- Dependency graph: boligstøtte evalueres *efter* kontanthjælp
- Cirkulære afhængigheder detekteres ved init → returnerer `Err(EngineError::CyclicDependency { .. })` med tydelig fejlbesked
- Alle 3 regler implementeret og individuelt testede
- Kontanthjælp-beregning matcher 4 håndberegnede cases (enlig u/børn, enlig m/børn, par u/børn, par m/børn)
- Parametre er eksplicitte og konfigurerbare: `kontanthjaelp_basis_enlig: f64 = 12_550.0`
- EvalContext akkumulerer resultater så downstream-regler kan referere upstream
- **Ingen forklaringer.** Batch-resultater er kompakte.

**Implementering (Gold):**
- `rule_engine.rs` — trait `Rule { fn evaluate(&self, borger: &BorgerView, ctx: &EvalContext) -> RuleResult }`
- `dependency_graph.rs` — topologisk sortering, cycle detection → `Result<DepGraph, EngineError>`
- `rules/kontanthjaelp.rs`, `rules/boligstoette.rs`, `rules/boerne_ydelse.rs`
- `eval_context.rs` — akkumulerer resultater for downstream-regler
- `error.rs` — `EngineError` enum med `CyclicDependency`, `InvalidParameter`, etc. (thiserror)

**Done:** Motor der evaluerer 3 regler i korrekt rækkefølge. Compact output. Fejl er Result, ikke panic.

---

#### Ward 3 — Batch-evaluering & Kerneaggregering

**Formål:** Evaluer alle 100K borgere. Aggreger kun features der bruges af 2+ views (rule-of-two).

**Tests (Red):**
- Batch-eval af 100K completerer under 50ms (release)
- Kerne-aggregering: total beløb per regel, antal eligible, gennemsnit
- Segmentering: group-by på kommune_id og husstandstype
- Top-N: de N borgere med størst ydelse per regel (partial sort, ikke fuld sort)
- Resultater er deterministiske
- Memory: batch-resultat for 100K target under 10MB (verificeres med eksplicit memory accounting)
- CompactBorgerResult layout er verificeret med `std::mem::size_of` test

**Implementering (Gold):**
- `batch_evaluator.rs` — itererer BorgerStore, evaluerer regelkæde, akkumulerer
- `aggregator.rs` — sum, count, mean per regel, per segment
- `top_n.rs` — partial sort for top-N
- `CompactBorgerResult` — amounts + eligible flags, layout verificeret (ikke antaget)

**Done:** 100K borgere evalueret med fuld statistik. Baseline etableret.

---

#### Ward 4 — Scenarie-engine med Incremental Recompute & Diff

**Formål:** Simulatorens hjerte. Ændr en parameter → se forskellen. Den vigtigste ward.

**Tests (Red):**
- Kan oprette scenarie: kontanthjælp basissats +500 kr
- Parameter→root-rule mapping er eksplicit konfigureret (ikke håndkodet if-chain)
- Dirty propagation følger dependency edges fra affected root rules
- Kun dirty regler re-evalueres: ændring af kontanthjælp re-evaluerer kontanthjælp + boligstøtte, men IKKE børneydelse
- Incremental recompute er mindst 2x hurtigere end fuld re-eval
- Diff producerer: antal borgere der ændrer eligibility, total merudgift/besparelse
- Diff per segment: kommune → merudgift
- Top-N mest påvirkede borgere (|delta| ranked), returneret med borger_id
- End-to-end: parameter-ændring → diff-resultat under 100ms for 100K

**Implementering (Gold):**
- `scenario.rs` — parameter overrides med eksplicit `param_id → Vec<RuleId>` mapping
- `incremental.rs` — dirty flags propageret via dependency graph; unchanged rules reuse baseline results
- `diff.rs` — sammenligner baseline vs. scenarie → `DiffResult`
- `DiffResult` — per-regel aggregat-diff, per-segment diff, top-N med borger_id

**Done:** "Ændr kontanthjælp +500 kr → 12.400 borgere påvirkes, merudgift 6.2M kr/md, Aalborg +820K kr" — under 100ms.

---

### ⛩ Epic 1 Gate — Multi-model review

| Reviewer | Fokus |
|----------|-------|
| Claude | Arkitektur, ward boundaries, rule-of-two compliance |
| GPT | Domæne-plausibilitet, scope creep, performance-estimater |

---

### Epic 2: Bridge & Views (Ward 5–7)

---

#### Ward 5 — WASM Bridge

**Formål:** Broen. JS kan kalde Rust og modtage data. Simpelt og pragmatisk.

**API:**

| Funktion | Input | Output | Bruges af |
|----------|-------|--------|-----------|
| `init()` | — | baseline stats | Alle |
| `get_baseline_stats()` | — | per-regel aggregat | Dashboard, Geo |
| `apply_scenario(param_id, value)` | param enum + f64 | DiffResult JSON | Alle |
| `get_top_affected(n)` | count | Vec\<{borger_id, delta}\> | Dashboard, Explainer |
| `get_case_detail(borger_id)` | borger_id: u32 | On-demand forklaring | Explainer |
| `get_geo_data()` | — | per-kommune diff | Geo |
| `get_filtered_stats(kommune_id)` | kommune_id: Option\<u8\> | filtreret aggregat | Dashboard (post geo-klik) |

**Tests (Red):**
- Alle API-funktioner callable fra JS og returnerer valid JSON
- `init()` genererer data og evaluerer baseline under 2 sekunder
- `apply_scenario()` returnerer diff under 100ms
- `get_case_detail(borger_id)` genererer on-demand forklaring under 5ms
- Round-trip (JS→WASM→JS) overhead under 2ms
- WASM binary under 1MB gzipped (stretch: under 500KB)

**Implementering (Gold):**
- `wasm_api.rs` — `#[wasm_bindgen]` funktioner der wrapper engine
- JSON valgt i v1 for simplicity og debuggability. Kun på public bridge — intern engine bruger typed Rust structs, aldrig JSON. Kan erstattes med binary protocol hvis transfer bliver målbar flaskehals.
- `bridge.js` — ~30 linjer: loader WASM, eksponerer async API

**Done:** JS kalder Rust, får data. Intet mere, intet mindre.

---

#### Ward 6 — Dashboard View

**Formål:** Hovedskærmen. Stats, sliders, histogram, diff-overlay. Vanilla JS + SVG.

**Tests (Red):**
- Stats-panel: total udgift, antal eligible, gennemsnit per regel
- 4 parameter-sliders med labels og aktuel værdi
- Slider-ændring → WASM-kald → stats opdateres under 200ms end-to-end
- Debounce: input events throttled til 16–32ms, final value commits immediately på pointerup
- Histogram: SVG, 20 buckets, viser ydelsesfordeling med baseline/scenarie overlay
- Histogram opdateres visuelt under 200ms; animation er optional og respekterer `prefers-reduced-motion`
- Top-10 mest påvirkede borgere som klikbar liste (borger_id baseret)
- Segment-breakdown: tabel med per-husstandstype aggregering
- Aktivt kommune-filter (fra geo-klik) vises og kan nulstilles
- Responsivt layout, mørkt tema via CSS custom properties

**Implementering (Gold):**
- `dashboard.js` — orkestrerer views, slider events med debounce
- `stats_panel.js` — DOM-manipulation for headline-tal
- `histogram.js` — SVG fra bucket-data, baseline+scenarie overlay
- `slider_panel.js` — input[type=range], dansk talformatering
- `affected_list.js` — top-N med click→case explainer
- `segment_table.js` — HTML table
- `styles.css` — CSS custom properties, responsive grid, mørkt tema
- ~300 linjer total JS

**Done:** Slider bevæges → tal ændrer sig → histogram opdateres → borgere rangeres. Under 200ms.

---

#### Ward 7 — Case Explainer View

**Formål:** Klik på én borger → se præcis hvordan reglerne ramte. On-demand, ikke batch. Killer-featuren.

**Tests (Red):**
- `get_case_detail(borger_id)` returnerer borgerdata + per-regel trace med forklaringstekst
- Forklaringer er menneskelige: "Kontanthjælp: Enlig over 30, basissats 12.550 kr. Forsørgertillæg: 2 børn → +3.420 kr."
- Diff-view: baseline vs. scenarie for denne borger
- Highlighter reglen med størst |delta|
- Dependency graph: simpel SVG med 3 bokse + pile
- Tal i dansk format: 12.550,00 kr
- Outlier-indikator: "Tæt på boligstøtte-tærskel (±3%)" — threshold proximity er konfigurerbar i explainer-modul (default 3%)
- Panel åbner som overlay uden at resette dashboard-state
- Lukker med Escape eller klik udenfor

**Implementering (Gold):**
- `case_explainer.rs` (Rust, explainer-modul) — generevaluerer én borger med fuld trace + forklaringsstrenge
- `threshold_detector.rs` (Rust, explainer-modul) — proximity til grænseværdier
- `case_panel.js` — DOM overlay med borgerdata, regelkæde, diff
- `rule_graph.js` — simpel SVG: 3 bokse med pile
- ~50 linjer Rust (on-demand), ~150 linjer JS

**Done:** "Borger #47.891: Maria, 34, enlig, 2 børn. Kontanthjælp: 15.970 → 16.470 kr (+500 kr). Boligstøtte: 3.200 → 3.050 kr (-150 kr, indkomstgrundlag steg). OBS: 2.8% fra boligstøtte-grænsen."

---

### ⛩ Epic 2 Gate — Multi-model review

| Reviewer | Fokus |
|----------|-------|
| Claude | API-design, render-pragmatik, ward boundaries |
| GPT | DX, scope creep, JS-lag kompleksitet |

---

### Epic 3: Geo View & Performance (Ward 8–10)

---

#### Ward 8 — Geo View (Kommunekort)

**Formål:** Danmark-kortet. Simpelt SVG med 98 paths der skifter farve. Ikke et GIS-system. Simpelt, grimt nok, tydeligt nok, hurtigt nok.

**Tests (Red):**
- `get_geo_data()` returnerer per-kommune diff (merudgift/besparelse, antal påvirkede, per-capita)
- SVG med 98 kommune-paths renderes korrekt
- Heatmap-farveskala: grøn (besparelse) → grå (neutral) → rød (merudgift)
- Opdateres ved scenarie-ændring under 200ms
- Hover → tooltip med kommunenavn og nøgletal
- Click → sætter aktivt kommune-filter → dashboard rekvirerer filtered data fra WASM via `get_filtered_stats(kommune_id)`
- Case explainer og top-N følger aktivt filter
- UI viser tydeligt at data er filtreret (badge/label med kommunenavn + nulstil-knap)
- Per-capita toggle

**Implementering (Gold):**
- `geo_aggregator.rs` (Rust, geo-modul) — group-by kommune_id, diff per kommune
- `kommune_kort.js` — indlejret SVG (98 forenklet paths, ~40KB), fill-color fra data
- `tooltip.js` — positioneret div
- Kommune-filter state delt med dashboard

**Done:** Slider bevæges → 98 kommuner skifter farve → klik på Aalborg → dashboard viser kun Aalborg-data.

---

#### Ward 9 — Performance Tuning & 500K Stretch

**Formål:** Fra "det virker" til "det er absurd hurtigt." Faktiske benchmarks, ikke estimater.

**Tests (Red):**
- 100K batch-eval: under 30ms (release, optimeret)
- 100K incremental scenarie: under 15ms
- 500K batch-eval: under 150ms (stretch)
- 500K incremental: under 80ms (stretch)
- WASM→JS data transfer: under 5ms
- DOM update: under 16ms (60fps)
- End-to-end slider → visual: under 100ms for 100K, under 200ms for 500K
- Memory: under 50MB for 100K, under 250MB for 500K
- WASM binary: under 1MB gzipped (stretch: under 500KB)

**Implementering (Gold):**
- SIMD (wasm-simd128) for batch-eval hot loops — kun hvis profiling viser at det er flaskehalsen, ikke som ritual
- struct-of-arrays verificering — profiler for AoS-regression
- wasm-opt -O3 med strip
- Profile hot paths med browser devtools
- Compact JSON: kun ændrede data ved incremental update

**Done:** Performance-tal der kan stå i en præsentation. Faktiske, målte tal.

---

#### Ward 10 — Accessibility & Error Handling

**Formål:** Demo-professionelt, ikke fuld enterprise. Nok til at det ikke er pinligt.

**Tests (Red):**
- Keyboard-navigation: alle sliders, alle klikbare borgere, kommune-kort
- Screen reader: stats-panel har aria-live="polite", slider-labels korrekte
- WCAG 2.1 AA farvekontrast på tekstelementer
- Loading state: synlig indikation mens WASM initialiserer
- Error boundary: graceful besked hvis WASM ikke supporteres
- `prefers-reduced-motion` respekteret på alle animationer
- Focus trap i case explainer, focus restore ved lukning

**Implementering (Gold):**
- ARIA-attributter på interaktive elementer
- Fallback-besked for browsere uden WASM
- Loading spinner/skeleton ved init

**Done:** Det opfører sig professionelt. Ikke enterprise-complete, men ikke amatøragtigt.

---

### ⛩ Epic 3 Gate — Multi-model review

| Reviewer | Fokus |
|----------|-------|
| Claude | Performance-benchmarks, a11y compliance |
| Gemini | Geo-data, alternative optimeringsstrategier |

---

### Epic 4: Demo (Ward 11–12)

---

#### Ward 11 — Demo Orchestration

**Formål:** Den demo du viser Opus Personale-teamet.

**Tests (Red):**
- Appen loader under 3 sekunder (inkl. WASM + data-generering)
- Startskærm med kort intro og disclaimer + "Start simulering"
- Pre-loaded demo-scenarie: "Kontanthjælp basissats +500 kr"
- Performance-badge i hjørnet (toggles med P-tast)
- View-switching: 1/2/3 tastatur-genveje
- Fullscreen-mode (F-tast)
- Alle 3 views fungerer og er forbundet

**Implementering (Gold):**
- `app.js` — orkestrerer WASM-init, view-switching, demo-state, filter-state
- Intro-overlay med disclaimer
- Performance HUD: ms for seneste eval, antal borgere, WASM heap

**Done:** Åbn URL → se borgere → ryk slider → alt ændrer sig → klik borger → se hvorfor → klik Danmark → se hvor. Under 100ms.

---

#### Ward 12 — Dokumentation & Video

**Formål:** Ship it.

**Tests (Red):**
- README.md med: projektbeskrivelse, arkitektur, screenshot, build-instruktioner
- ARCHITECTURE.md med: rule-of-two, dependency graph, module map
- Performance-tabel med **faktiske** benchmarks
- Ward-log: faktisk tid per ward vs. estimat

**Implementering (Gold):**
- README.md
- ARCHITECTURE.md
- Benchmarks tabel med reelle tal fra Ward 9
- Ward-log: ærlig tidsrapportering
- Video: screen recording + voice-over, 10-15 min (kan laves efter demo)

**Done:** Projektet er komplet, dokumenteret, kan vises til enhver, og ward-loggen dokumenterer WDD i praksis.

---

### ⛩ Epic 4 Gate — Multi-model review

| Reviewer | Fokus |
|----------|-------|
| GPT | Demo-flow, storytelling, huller |
| Gemini | Dokumentationskvalitet, onboarding-oplevelse |

---

## MoSCoW

### Must Have
- Ward 1–5: Engine + bridge
- Ward 6: Dashboard
- Ward 7: Case explainer
- Ward 9: Performance tuning
- Ward 11: Demo orchestration

### Should Have
- Ward 8: Geo view
- Ward 10: Accessibility

### Could Have
- Ward 12: Dokumentation & video
- 500K stretch (Ward 9)

---

## Tidslinje

| Epic | Wards | Estimat |
|------|-------|---------|
| Engine Foundation | 1–4 | 2 dage |
| Bridge & Views | 5–7 | 2 dage |
| Geo & Performance | 8–10 | 1.5 dage |
| Demo | 11–12 | 0.5–1 dag |

**Must-have path: ~5.5 dage**
**Full plan: ~6.5 dage**

---

## Video-plan

| Segment | Indhold | Target |
|---------|---------|--------|
| Intro | "Hvad hvis 100.000 borgersager kunne evalueres i din browser?" | 30 sek |
| Epic 1 | Rust-engine, tests, data, benchmark | 3 min |
| Epic 2 | Bridge + views, "der er ingen React" | 3 min |
| Case Explainer | Klik på én borger, se alt | 2 min |
| Danmark-kortet | Slider → kommuner skifter farve | 1 min |
| Fuld demo | Alt kører, sliders, drilldown, geo-filter | 2 min |
| Outro | Ward count, test count, tid, benchmarks | 30 sek |

**Target: 12 minutter.**

---

## Pitch

> **kmd-regelsim er en browser-native konsekvenssimulator der på syntetiske borgerdata kan vise effekten af regelændringer i realtid — samlet, segmenteret og for den enkelte case — uden backend og uden at data forlader maskinen.**

Tech-kicker:

> **100.000 borgere. 3 ydelsesregler. 4 justerbare parametre. Under 100ms fra slider til skærm. Bygget af én udvikler på én uge med Ward-Driven Development.**

---

## v1.1 Roadmap — Overenskomstforhandling (Opus Forhandling)

**Ikke i v1-scope. Dokumenteret her som næste domæne-slice.**

### Baggrund

KMD Opus Forhandling er et eksisterende modul bygget i ældre SAP-teknologi, brugt af kommuner og regioner i forbindelse med overenskomstforhandlinger. Forhandlerne skal kunne besvare spørgsmål som: "Hvad koster det at hæve grundlønnen i løngruppe 7 med 1.200 kr?" og "Hvor mange ansatte rammer pensionsgrænsen?"

Det er *præcis* samme problemtype som kmd-regelsim løser for ydelsesregler. Bare med et andet regelsæt.

### Hvorfor det passer

| kmd-regelsim (v1) | Opus Forhandling (v1.1) |
|--------------------|-------------------------|
| Borgere med ydelsesprofiler | Ansatte med lønprofiler |
| Kontanthjælp, boligstøtte, børneydelse | Grundløn, pension, ferietillæg, anciennitetstrin |
| "Ændr basissats +500 kr" | "Hæv grundløn løngruppe 7 +1.200 kr" |
| Downstream: boligstøtte ændres | Downstream: pensionsbidrag, feriepenge ændres |
| Per-kommune fordeling | Per-forvaltning/afdeling fordeling |
| Case explainer: én borger | Case explainer: én ansat |
| Outlier: tæt på tærskel | Outlier: rammer pensionsgrænse, skifter løntrin |

Motoren er identisk. Rule trait, dependency graph, batch-eval, incremental recompute, scenarie-diff, case explainer. Reglerne er konfiguration, ikke kode.

### Hvad v1.1 kræver

**Nyt regelsæt (Rust):**

| Regel | Parametre | Afhænger af |
|-------|-----------|-------------|
| Grundløn | Løngruppe, anciennitetstrin, beskæftigelsesgrad | — (root) |
| Pensionsbidrag | Procentsats, grænsebeløb, karensperiode | Grundløn |
| Ferietillæg | Procent af årsløn, særlig feriegodtgørelse | Grundløn |
| Kvalifikationstillæg | Funktions-/kvalifikationsløn, lokale aftaler | — (root) |

**Ny syntetisk data:**
- 50K–100K ansatte med løngruppe, anciennitet, forvaltning, afdeling, beskæftigelsesgrad
- Realistiske fordelinger baseret på offentlige overenskomstdata

**Genbrugt fra v1 (uændret):**
- kmd-engine kernen (batch-eval, incremental, diff, top-N, segmentering)
- WASM bridge
- Dashboard view (sliders, stats, histogram, segment-tabel)
- Case explainer (on-demand forklaring, threshold detection)
- Geo view (per-forvaltning i stedet for per-kommune)
- Performance tuning
- Demo orchestration

**Nyt JS (minimalt):**
- Slider-labels og parameternavne
- Forvaltnings-/afdelingsview i stedet for kommunekort (eller genbruge kort med anden data)

### Estimat

2–3 dage ovenpå v1. Det meste er nye regler og ny datagenerering. Motoren og UI'et er allerede bygget.

### Strategisk værdi

Det er her kmd-regelsim går fra "fed tech-demo" til "platform." Hvis du kan demonstrere at samme motor håndterer både ydelsesberegning OG overenskomstforhandling, har du bevist at det ikke er et engangsprojekt — det er en arkitektur.

Og pitchen til Martin bliver: "Den SAP-baserede Opus Forhandling I har i dag kører server-side med roundtrips for hver beregning. Her er den samme funktionalitet, i browseren, i realtid, på syntetiske data. Bygget på 2 dage ovenpå den eksisterende motor."

**Taktisk note:** Vis ydelsessimulatoren først. Lad teamet blive imponerede. Drop *derefter*: "I øvrigt kender I Opus Forhandling?" Lad dem koble prikkerne selv. Det er stærkere end at fortælle dem.

---

## Changelog

| Version | Dato | Ændringer |
|---------|------|-----------|
| 1.0 | 14. mar 2026 | Initial plan, 14 wards, 8 regler |
| 2.0 | 14. mar 2026 | Post GPT 5.4 review runde 1: 3 regler, on-demand forklaringer, rule-of-two, 12 wards, 100K baseline |
| 3.0 | 14. mar 2026 | Post GPT 5.4 review runde 2: Result over panic, eksplicit param→rule mapping, JSON som demo-transport, borger_id i API, kommune-filter spec, WASM binary 1MB target, dependency policy, Arrow-beslutning, north star |
| 3.1 | 14. mar 2026 | v1.1 roadmap: Opus Forhandling som næste domæne-slice, rule-of-two validering af motor-generiskhed |
| 3.2 | 14. mar 2026 | Final polish: Non-goals, JSON-regel, splittet succesmåling, debounce-spec, konfigurerbar threshold, filter-visibility, SIMD som værktøj ikke ritual, platform-safe determinism |

---

*"Boring is beautiful. But this time, it's also spectacular."*