# Contributing to the Go/Rust/Bash SAST Benchmark

Thank you for your interest in improving SAST tool accuracy for Go, Rust, and Bash. This benchmark exists because no public ground truth existed for these languages. Every contribution helps the entire SAST ecosystem.

## How to Contribute

### Adding Test Cases

Each test case is a single source file with one function. The function does something realistic -- handles an HTTP request, processes data, writes to a file -- and is either vulnerable or safe.

**Requirements for new test cases:**

1. **One file, one function.** File name: `benchmark_test_NNNNN.go` (or `.rs`, `.sh`). Function name: `BenchmarkTestNNNNN`.

2. **No vulnerability hints.** Do not add comments like "VULNERABLE", "TAINT SINK", "INSECURE", or "SAFE". The code should look like normal production code a developer would write. The CSV is the only ground truth.

3. **CSV entry required.** Every test file must have a corresponding entry in `expectedresults-0.1.csv`:
   ```
   BenchmarkTestNNNNN,category,true/false,CWE
   ```

4. **Pair vulnerable with safe.** For every new vulnerable pattern, consider adding a corresponding safe variant that looks similar but uses proper mitigation. This tests FP rejection.

5. **Use the shared helpers.** Each language has a `shared.go` (or equivalent) with DB connection and response utilities. Use them for consistency.

6. **Realistic code.** Test cases should resemble code a real developer would write, not contrived edge cases. Draw inspiration from CVEs, security advisories, and real-world codebases.

### CWE Categories

Use existing categories where possible. If adding a new CWE category, update the language's benchmark documentation file with the new category, CWE number, and description.

Current Go categories: sqli(89), cmdi(78), pathtraver(22), xss(79), ssrf(918), weakrand(330), weakhash(328), weakcipher(327), securecookie(614), redirect(601), tlsverify(295), deserial(502), loginjection(117), nosql(943), ldapi(90), trustbound(501)

### Test Case Design Principles

**What makes a GOOD vulnerable test case:**
- Uses a real API that developers actually use (not a toy function)
- The vulnerability is non-obvious -- not just `eval(userInput)`
- Multi-hop flows, cross-function calls, struct field propagation
- Patterns from real CVEs and security advisories

**What makes a GOOD safe test case:**
- Looks similar to a vulnerable variant (hard FP test)
- Uses a real mitigation technique (parameterized queries, input validation, allowlists)
- Not trivially safe (e.g., hardcoded constant with no user input)

**What makes a BAD test case (don't do this):**
- Comments explaining the vulnerability
- Contrived patterns nobody would write in production
- Safe variants that are obviously different from vulnerable ones
- Test cases that require runtime behavior to determine safety

### Reporting Issues

If you find a test case that is misclassified (vulnerable marked as safe or vice versa), please open an issue with:

1. Test case number (e.g., BenchmarkTest00042)
2. Current classification in CSV
3. Why you believe it's wrong
4. Evidence (code snippet + explanation)

### Improving Documentation

Each language directory has a `*_benchmark.md` file. Improvements to scoring scripts, category descriptions, or pattern documentation are welcome.

## Code Style

- **Go:** Standard `gofmt` formatting. Package `testcode`. Use `shared.go` helpers.
- **Rust:** Standard `rustfmt` formatting. Module structure per benchmark doc.
- **Bash:** ShellCheck-clean. POSIX-compatible where possible.

## Validation

Before submitting, run the validation script to ensure CSV/file consistency:

```bash
python scripts/validate_go.py    # Go benchmark
python scripts/validate_rust.py  # Rust benchmark
python scripts/validate_bash.py  # Bash benchmark
```

The validator checks:
- Every CSV key has a matching test file
- Every test file has a matching CSV entry
- No duplicate keys
- Per-category TP/TN balance summary

## Scoring Your Tool

To score a SAST tool against the benchmark:

```bash
# Export SARIF from your tool
your-tool scan go/testcode/ --output results.sarif

# Score
python scripts/score_sarif.py results.sarif go/expectedresults-0.1.csv
```

See `go/SCORING.md` for tool-specific SARIF export commands.

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b add-go-race-condition-tests`)
3. Add test files + CSV entries
4. Run `python scripts/validate_go.py` to verify consistency
5. Update the language's benchmark doc if adding new categories
6. Submit a PR with a clear description of what patterns you're testing and why

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
