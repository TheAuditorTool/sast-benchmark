# Ruby Benchmark Baseline -- TheAuditor

**Date**: Pending
**Version**: Pending initial run
**Category-Averaged Score (OWASP)**: Pending
**Flat Score**: Pending

## Category Scores

| Category | CWE | TP | FP | FN | TN | TPR | FPR | Score |
|----------|-----|----|----|----|----|----|-----|-------|
| (Pending initial TheAuditor run) | | | | | | | | |

## Notes

This file will be populated after the first TheAuditor run against the completed Ruby benchmark.
Run: `cd ruby/ && aud full --offline && python ../scripts/convert_theauditor.py .pf/repo_index.db --language ruby --benchmark-dir . > theauditor.sarif && python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.1.0.csv`
