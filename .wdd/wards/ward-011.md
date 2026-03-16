---
ward: 11
revision: null
name: "Demo Orchestration"
epic: "Demo"
status: "planned"
dependencies: [6, 7, 8, 9, 10]
layer: "javascript"
estimated_tests: 7
created: "2026-03-16"
completed: null
---
# Ward 011: Demo Orchestration

## Scope
Den demo du viser Opus Personale-teamet. App-shell, view-switching, intro-overlay, filter-state, performance HUD, keyboard shortcuts.

## Inputs
- Alle views fra Ward 6-8
- Performance-optimeret engine fra Ward 9
- A11y fra Ward 10

## Outputs
- `app.js` — orkestrerer WASM-init, view-switching, demo-state, filter-state
- Intro-overlay med disclaimer
- Performance HUD (toggle)
- Keyboard shortcuts (1/2/3, P, F)

## Specification

### Features
1. **Load:** Under 3 sekunder inkl. WASM + data-generering
2. **Intro-overlay:** Kort tekst + disclaimer + "Start simulering"-knap
3. **Pre-loaded scenarie:** "Kontanthjælp basissats +500 kr"
4. **Performance HUD:** ms for seneste eval, antal borgere, WASM heap (toggle med P)
5. **View-switching:** 1/2/3 tastatur-genveje
6. **Fullscreen:** F-tast
7. **Filter-state:** Alle views forbundet, kommune-filter propagerer

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_load_time | Appen loader under 3 sekunder |
| 2 | test_intro_overlay | Startskærm med intro, disclaimer, start-knap |
| 3 | test_preloaded_scenario | Pre-loaded demo-scenarie fungerer |
| 4 | test_performance_hud | Performance-badge toggles med P-tast |
| 5 | test_all_views_connected | Alle 3 views fungerer og er forbundet |
| 6 | test_keyboard_shortcuts | 1/2/3, P, F tastatur-genveje |
| 7 | test_filter_state_propagates | Kommune-filter propagerer til alle views |

## Must NOT
- Ændre view-implementeringerne
- Hardcode data i demo-mode

## Must DO
- Under 3 sekunder load
- Intro-overlay med disclaimer
- Filter-state forbinder alle views
- Keyboard shortcuts + performance HUD

## Verification
Åbn URL → intro → start → slider → under 100ms → klik borger → forklaring → klik kommune → filter propagerer.
