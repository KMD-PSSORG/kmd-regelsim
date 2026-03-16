---
ward: 7
revision: null
name: "Case Explainer View"
epic: "Bridge og Render Layer"
status: "complete"
dependencies: [5, 6]
layer: "mixed"
estimated_tests: 9
created: "2026-03-16"
completed: "2026-03-16"
---
# Ward 007: Case Explainer View

## Scope
Killer-featuren. Klik på én borger → se præcis hvordan reglerne rammer. On-demand via borger_id. Konfigurerbar threshold proximity.

## Inputs
- WASM bridge (`get_case_detail(borger_id)`) fra Ward 5
- Dashboard view (klik på borger) fra Ward 6

## Outputs
- `case_explainer.rs` (Rust, explainer-modul) — generevaluerer én borger med fuld trace + forklaringsstrenge
- `threshold_detector.rs` (Rust, explainer-modul) — proximity til grænseværdier, konfigurerbar (default 3%)
- `case_panel.js` — DOM overlay med borgerdata, regelkæde, diff
- `rule_graph.js` — simpel SVG med 3 bokse + pile (dependency visualization)
- ~50 linjer Rust (on-demand), ~150 linjer JS

## Specification

### Forklaringsformat
"Kontanthjælp: Enlig over 30, basissats 12.550 kr. Forsørgertillæg: 2 børn → +3.420 kr."

### Features
1. **Borgerdata:** Alle felter formateret, identificeret via borger_id
2. **Per-regel trace:** Beløb + menneskelig forklaringstekst
3. **Diff-view:** Baseline vs. scenarie for denne borger
4. **Highlight:** Reglen med størst |delta|
5. **Dependency graph:** SVG med 3 bokse + pile
6. **Dansk formatering:** 12.550,00 kr
7. **Overlay:** Åbner uden at resette dashboard-state. Lukker med Escape eller klik udenfor.
8. **Outlier-indikator:** Threshold proximity konfigurerbar i explainer-modul (default 3%)

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_case_detail_by_borger_id | get_case_detail(borger_id) returnerer borgerdata + per-regel trace |
| 2 | test_human_readable_explanations | Forklaringer er menneskelige, ikke tekniske |
| 3 | test_diff_view | Baseline vs. scenarie for denne borger |
| 4 | test_highlight_largest_delta | Highlighter reglen med størst |delta| |
| 5 | test_dependency_graph_svg | SVG med 3 bokse og pile |
| 6 | test_danish_formatting | Tal i dansk format: 12.550,00 kr |
| 7 | test_overlay_escape_close | Panel lukker med Escape og klik udenfor, dashboard-state bevaret |
| 8 | test_outlier_configurable_threshold | Outlier-indikator med konfigurerbar threshold (default 3%) |
| 9 | test_follows_kommune_filter | Case explainer følger aktivt kommune-filter |

## Must NOT
- Batch-generere forklaringer for alle borgere
- Ændre dashboard-state når panel åbner/lukker
- Hardcode threshold-procent

## Must DO
- On-demand via borger_id: kun 1 borger
- Under 5ms for case detail
- Dansk talformatering
- Konfigurerbar threshold (default 3%)
- Escape/click-outside close

## Verification
Klik borger → "Maria, 34, enlig, 2 børn. Kontanthjælp: 15.970 → 16.470 kr (+500 kr). OBS: 2.8% fra boligstøtte-grænsen." Escape lukker.
