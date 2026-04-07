# Vulnerable Apps Benchmark Changelog

## v0.1.0 (2026-04-08)

### Initial Release

26 reference applications centralized from per-language `apps/` directories into `vulnerable_apps/` with standalone scoring infrastructure.

- **Go**: 6 apps, 394 ground truth entries (per-app `ground_truth.csv`, function-level)
- **Rust**: 8 apps, 119 entries (`expectedresults-0.1.0.csv`, annotation-based)
- **PHP**: 4 apps, 118 entries (`expectedresults-0.1.0.csv`, annotation-based)
- **Bash**: 5 apps, 191 entries (`expectedresults-0.1.0.csv`, annotation-based)
- **Ruby**: 3 apps, 62 entries (`expectedresults-0.1.0.csv`, annotation-based)

Per-language benchmark docs (`{lang}_apps_benchmark.md`), validation script (`validate_vulnerable_apps.py`).

Previously these apps were mixed into the main benchmark CSVs alongside standalone testcode. Separated because apps use different scoring mechanisms (annotation-based multi-function-per-file vs filename-based 1-file-1-test).
