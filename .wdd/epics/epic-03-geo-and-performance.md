# Epic 03: Geo View og Performance

## Goal
Tilføj Danmark-kort med 98 kommuner som heatmap, optimér performance til absurd hurtigt, og tilføj accessibility og error handling for produktionskvalitet.

## Wards
| Ward | Name | Status |
|------|------|--------|
| 8 | Geo View Kommunekort | planned |
| 9 | Performance Tuning og 500K Stretch | planned |
| 10 | Accessibility og Error Handling | planned |

## Integration Points
- Ward 8 afhænger af bridge (Ward 5) og dashboard integration (Ward 6)
- Ward 9 optimerer engine fra Epic 1 og bridge fra Ward 5
- Ward 10 forbedrer alle views fra Ward 6-8

## Completion Criteria
- 98 kommuner skifter farve ved slider-ændring under 200ms
- 100K batch-eval under 30ms, scenarie-diff under 15ms
- Keyboard-nav, screen reader support, WCAG AA
