---
ward: 10
revision: null
name: "Accessibility og Error Handling"
epic: "Geo View og Performance"
status: "planned"
dependencies: [6, 7, 8]
layer: "javascript"
estimated_tests: 7
created: "2026-03-16"
completed: null
---
# Ward 010: Accessibility og Error Handling

## Scope
Demo-professionelt, ikke fuld enterprise. Nok til at det ikke er pinligt. Keyboard-nav, screen reader, WCAG AA, loading states, error boundaries, focus management.

## Inputs
- Alle 3 views fra Ward 6-8

## Outputs
- ARIA-attributter på alle interaktive elementer
- Keyboard-navigation
- Loading state under WASM init
- Error boundary for manglende WASM support
- Focus trap + restore i case explainer
- prefers-reduced-motion support

## Specification

### Accessibility
1. **Keyboard:** Alle sliders, klikbare borgere, kommune-kort navigerbare
2. **Screen reader:** aria-live="polite" på stats-panel, korrekte slider-labels
3. **WCAG 2.1 AA:** Farvekontrast på tekstelementer
4. **Focus management:** Focus trap i case explainer, focus restore ved lukning

### Error Handling
1. **Loading state:** Synlig indikation mens WASM initialiserer (spinner/skeleton)
2. **WASM fallback:** Graceful besked hvis WASM ikke supporteres
3. **prefers-reduced-motion:** Alle animationer respekterer

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_keyboard_navigation | Alle interaktive elementer keyboard-tilgængelige |
| 2 | test_screen_reader | aria-live="polite", slider-labels korrekte |
| 3 | test_color_contrast | WCAG 2.1 AA farvekontrast |
| 4 | test_loading_state | Synlig indikation under WASM init |
| 5 | test_wasm_error_boundary | Graceful besked uden WASM |
| 6 | test_reduced_motion | prefers-reduced-motion respekteret på alle animationer |
| 7 | test_focus_trap_and_restore | Focus trap i case explainer, restore ved lukning |

## Must NOT
- Ændre funktionalitet (kun a11y og error handling)
- Bryde eksisterende tests

## Must DO
- ARIA-attributter på alle interaktive elementer
- Focus trap + restore i case explainer
- Loading spinner/skeleton ved init
- Graceful WASM fallback

## Verification
Keyboard-only navigation. Screen reader annoncerer. Farvekontrast passerer. Focus trap fungerer.
