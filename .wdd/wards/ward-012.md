---
ward: 12
revision: null
name: "Dokumentation og Video"
epic: "Demo"
status: "planned"
dependencies: [11]
layer: "documentation"
estimated_tests: 4
created: "2026-03-16"
completed: null
---
# Ward 012: Dokumentation og Video

## Scope
Ship it. README, arkitekturdokumentation, faktiske performance-benchmarks, video med narrativ, ærlig ward-log.

## Inputs
- Komplet applikation fra Ward 11

## Outputs
- README.md med beskrivelse, arkitektur, screenshots, build-instruktioner
- ARCHITECTURE.md med rule-of-two, dependency graph, module map
- Performance-tabel med faktiske benchmarks (fra Ward 9)
- Ward-log med faktisk tid per ward vs. estimat
- Video (10-15 min) med screen recording + voice-over

## Specification

### README.md
- Projektbeskrivelse med north star
- Arkitekturdiagram (ASCII)
- Screenshot
- Build-instruktioner
- Performance-benchmarks (faktiske tal fra Ward 9)

### ARCHITECTURE.md
- Rule-of-two forklaring + tabel
- Dependency graph
- Module map
- Designbeslutninger

### Video
- Target: 12 minutter
- "Hvad hvis 100.000 borgersager kunne evalueres i din browser?"

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_readme_complete | README.md med alle sektioner |
| 2 | test_benchmarks_actual | Performance-tabel med faktiske (ikke estimerede) benchmarks |
| 3 | test_video_exists | Video er optaget |
| 4 | test_ward_log | Ward-log med faktisk tid vs. estimat |

## Must NOT
- Bruge estimerede performance-tal
- Skrive dokumentation der ikke matcher koden

## Must DO
- Faktiske benchmarks fra Ward 9
- Ærlig ward-log tidsrapportering
- README med build-instruktioner

## Verification
README komplet. Benchmarks faktiske. Ward-log ærlig. Video har klar narrativ.
