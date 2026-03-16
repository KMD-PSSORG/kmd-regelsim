---
ward: 6
revision: null
name: "Dashboard View"
epic: "Bridge og Render Layer"
status: "planned"
dependencies: [5]
layer: "javascript"
estimated_tests: 10
created: "2026-03-16"
completed: null
---
# Ward 006: Dashboard View

## Scope
Hovedskærmen. Stats-panel, parameter-sliders med debounce, histogram med diff-overlay, top-10, segment-breakdown, aktivt kommune-filter display. Vanilla JS + SVG.

## Inputs
- WASM bridge API fra Ward 5

## Outputs
- `dashboard.js` — orkestrerer views, slider events med debounce
- `stats_panel.js` — DOM-manipulation for headline-tal
- `histogram.js` — SVG fra bucket-data, baseline+scenarie overlay
- `slider_panel.js` — input[type=range] med debounce, dansk talformatering
- `affected_list.js` — top-N med click→case explainer (borger_id baseret)
- `segment_table.js` — HTML table
- `styles.css` — CSS custom properties, responsive grid, mørkt tema
- ~300 linjer total JS

## Specification

### Komponenter
1. **Stats-panel:** Total udgift, antal eligible, gennemsnit per regel
2. **4 parameter-sliders:** kontanthjælp sats, forsørgertillæg, boligstøtte grænse, børneydelse aftrapning
3. **Debounce:** input events throttled til 16-32ms, final value commits immediately på pointerup
4. **Histogram:** SVG, 20 buckets, baseline/scenarie overlay
5. **Histogram animation:** Optional, respekterer `prefers-reduced-motion`
6. **Top-10 liste:** Mest påvirkede borgere med borger_id (klikbare → case explainer)
7. **Segment-breakdown:** Tabel med per-husstandstype aggregering
8. **Kommune-filter:** Aktivt filter (fra geo-klik) vises som badge/label med nulstil-knap

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_stats_panel_renders | Stats-panel viser total udgift, antal eligible, gennemsnit per regel |
| 2 | test_sliders_present | 4 parameter-sliders med labels og aktuel værdi |
| 3 | test_slider_triggers_update | Slider-ændring → WASM → stats under 200ms end-to-end |
| 4 | test_debounce | Input throttled 16-32ms, pointerup commits immediately |
| 5 | test_histogram_svg | SVG histogram med 20 buckets og baseline/scenarie overlay |
| 6 | test_reduced_motion | Histogram animation respekterer prefers-reduced-motion |
| 7 | test_top_10_list | Top-10 med borger_id og klik-handler |
| 8 | test_segment_table | Segment-breakdown per husstandstype |
| 9 | test_kommune_filter_display | Aktivt kommune-filter vises med badge og nulstil-knap |
| 10 | test_responsive_layout | Responsivt layout, mørkt tema |

## Must NOT
- Bruge React, Vue, eller andre frameworks
- Kalde Rust direkte — kun via bridge.js
- Generere forklaringer (det er Case Explainer, Ward 7)

## Must DO
- Vanilla JS + SVG
- Debounce: 16-32ms throttle, pointerup commit
- prefers-reduced-motion respekteret
- Kommune-filter badge med nulstil
- End-to-end slider → visual under 200ms

## Verification
Åbn browser. Sliders → tal → histogram → borgere. Under 200ms. Kommune-filter vises korrekt.
