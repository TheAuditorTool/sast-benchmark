# BenchProctor

A static application security testing (SAST) benchmark suite. BenchProctor provides
ground-truth corpora -- programs labeled vulnerable or safe -- so you can measure how
accurately any SAST tool detects real issues and how often it flags safe code.

Website: https://benchproctor.dev

## Why

A SAST tool is only as trustworthy as its accuracy, and accuracy is unmeasurable without
ground truth. For any tool, on languages and patterns with little or no public benchmark
coverage, BenchProctor answers:

- Does it detect the vulnerability? (true-positive rate)
- Does it flag safe code? (false-positive rate)
- Is it improving or regressing across versions?

## Scoring

Every test case carries a ground-truth label (`vulnerable` or `safe`) in a CSV answer key.
After a tool runs, scoring computes:

    TP = vulnerable and detected      FN = vulnerable and missed
    FP = safe and flagged             TN = safe and ignored

    TPR = TP / (TP + FN)              FPR = FP / (FP + TN)
    Score = TPR - FPR                 (Youden's J)

| Score | Meaning |
|------:|---------|
| +100% | Perfect -- catches everything, zero false alarms |
|    0% | No better than guessing |
| -100% | Flags safe code, misses vulnerable code |

Scores are reported two ways: category-averaged (each category weighted equally so large
categories can't dominate) and flat aggregate. The category-averaged score is the headline
number.

## Design principles

- Labels come from security knowledge, not from any tool's detection behavior.
- No hints in source: no vulnerability comments, CWE tags, or category names in filenames
  or identifiers. The CSV answer key is the only ground truth.
- Balanced corpora (about 50/50 vulnerable/safe) so a tool that flags everything scores 0%,
  not 100%.
- Tool-agnostic: any tool that emits SARIF 2.1.0 can be scored.

## Using it

    # 1. Run your SAST tool and export SARIF 2.1.0
    your-tool scan <corpus_dir> --format sarif -o results.sarif

    # 2. Score against the ground-truth CSV
    python scripts/score_sarif.py results.sarif <corpus_dir>/expectedresults-*.csv

The scorer uses the Python standard library only -- no dependencies.

## Status

BenchProctor is being prepared for its first public release. Corpora are versioned and
released separately; see https://benchproctor.dev for releases.

## License

Apache License 2.0 -- see [LICENSE](LICENSE).
