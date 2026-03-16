---
ward: 8
revision: null
name: "Geo View Kommunekort"
epic: "Geo View og Performance"
status: "planned"
dependencies: [5, 6]
layer: "mixed"
estimated_tests: 9
created: "2026-03-16"
completed: null
---
# Ward 008: Geo View Kommunekort

## Scope
Danmark-kortet. Simpelt SVG med 98 paths. Ikke et GIS-system. Click sætter aktivt kommune-filter der propagerer til dashboard og case explainer via WASM.

## Inputs
- WASM bridge (`get_geo_data()`, `get_filtered_stats(kommune_id)`) fra Ward 5
- Dashboard integration fra Ward 6

## Outputs
- `geo_aggregator.rs` (Rust, geo-modul) — group-by kommune_id, diff per kommune, per-capita
- `kommune_kort.js` — indlejret SVG (98 paths, ~40KB), fill-color fra data
- `tooltip.js` — positioneret div
- Kommune-filter state delt med dashboard

## Specification

### Features
1. **SVG-kort:** 98 kommune-paths med dynamisk fill
2. **Heatmap-farveskala:** Grøn (besparelse) → grå (neutral) → rød (merudgift)
3. **Opdatering:** Slider-ændring → nyt kort under 200ms
4. **Hover-tooltip:** Kommune-navn + nøgletal
5. **Click → filter:** Sætter aktivt kommune-filter → dashboard rekvirerer filtered data via `get_filtered_stats(kommune_id)`
6. **Case explainer følger filter:** Top-N og drilldown filtreret til valgt kommune
7. **Filter-UI:** Badge/label med kommunenavn + nulstil-knap
8. **Per-capita toggle:** Absolutte tal vs. per-borger

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_geo_data_returns | get_geo_data() returnerer per-kommune diff med per-capita |
| 2 | test_svg_98_paths | SVG med 98 kommune-paths renderes |
| 3 | test_heatmap_colors | Farveskala: grøn → grå → rød |
| 4 | test_update_on_scenario | Opdateres ved scenarie-ændring under 200ms |
| 5 | test_hover_tooltip | Hover → tooltip med navn og nøgletal |
| 6 | test_click_sets_filter | Click → sætter aktivt kommune-filter |
| 7 | test_dashboard_receives_filtered | Dashboard rekvirerer filtered data via get_filtered_stats |
| 8 | test_filter_badge_and_reset | UI viser filter-badge med nulstil-knap |
| 9 | test_per_capita_toggle | Toggle absolutte tal vs. per-borger |

## Must NOT
- Implementere fuldt GIS-system
- Bruge eksterne map-biblioteker
- Lade dashboard styre filter-logik (geo ejer sin click → filter sættes centralt)

## Must DO
- Click → get_filtered_stats(kommune_id) via WASM
- Case explainer og top-N følger aktivt filter
- Filter-badge synlig med nulstil
- Under 200ms opdatering

## Verification
Slider → 98 kommuner skifter farve → klik Aalborg → dashboard viser kun Aalborg-data → badge synlig → nulstil.
