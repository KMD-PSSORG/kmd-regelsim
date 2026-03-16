# Epic 02: Bridge og Render Layer

## Goal
Forbind Rust-engine til browseren via WASM bridge, og byg de tre views: Dashboard (hovedskærm), Case Explainer (borger-drilldown), og view-switching. Vanilla JS + SVG, intet framework.

## Wards
| Ward | Name | Status |
|------|------|--------|
| 5 | WASM Bridge | planned |
| 6 | Dashboard View | planned |
| 7 | Case Explainer View | planned |

## Integration Points
- Ward 5 afhænger af komplet engine fra Epic 1
- Ward 6-7 bruger bridge API fra Ward 5
- Ward 8 (Geo View) afhænger af bridge og dashboard fra denne epic

## Completion Criteria
- Slider bevæges → tal ændrer sig → histogram animerer → borgere rangeres. Alt under 200ms.
- Klik på borger → fuld forklaring med regelkæde, diff, outlier-detection.
