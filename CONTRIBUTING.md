# Contributing to the Go/Rust/Bash/PHP + Adversarial SAST Benchmark

Thank you for your interest in improving SAST tool accuracy. This benchmark exists because no public ground truth existed for Go, Rust, Bash, or PHP vulnerability detection -- or for adversarial evasion detection in any language. Every contribution helps the entire SAST ecosystem.

## How to Contribute

### Adding Test Cases

Each test case is a single source file with one function. The function does something realistic -- handles an HTTP request, processes data, writes to a file -- and is either vulnerable or safe.

**Requirements for new test cases:**

1. **One file, one function.** File name: `benchmark_test_NNNNN.go` (or `.rs`, `.sh`). Function name: `BenchmarkTestNNNNN`.

2. **No vulnerability hints.** Do not add comments like "VULNERABLE", "TAINT SINK", "INSECURE", or "SAFE". The code should look like normal production code a developer would write. The CSV is the only ground truth.

3. **CSV entry required.** Every test file must have a corresponding entry in the language's `expectedresults-<version>.csv`:
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

### Adding Adversarial Evasion Test Cases

Adversarial test cases are fundamentally different from language benchmark tests. They test **concealment detection**, not vulnerability detection.

**Structure:** Each test case uses `vuln-code-snippet` annotation markers:
```
// vuln-code-snippet start ADV-UC-001
// ... code with invisible Unicode payload ...
// vuln-code-snippet vuln-line ADV-UC-001
// vuln-code-snippet end ADV-UC-001
```

**Requirements for adversarial test cases:**

1. **Annotation markers required.** Every test case needs `start`/`end` markers plus either `vuln-line` (for TPs) or `safe-line` (for TNs).

2. **CSV entry required.** Add to `adversarial/expectedresults-0.1.0.csv`:
   ```
   ADV-XX-NNN,category,true/false,CWE
   ```

3. **Real attack patterns only.** TPs must represent a documented attack technique or a direct extrapolation of one. Cite the campaign, CVE, or research paper.

4. **Non-trivial true negatives.** TN cases must look functionally similar to TPs. A TN for `unicode_payload` should use legitimate Unicode (emoji, i18n text) -- not just ASCII code with no Unicode at all.

5. **Cross-language preferred.** If the attack works in JavaScript, add Python and Go variants. Most evasion techniques are language-agnostic.

6. **Categories:** `unicode_payload`, `visual_deception`, `dynamic_construction`, `supply_chain`, `ai_prompt_injection`, `c2_fingerprint`. Propose new categories via issue first.

See [adversarial/adversarial_benchmark.md](adversarial/adversarial_benchmark.md) for detailed category descriptions and design philosophy.

### Improving Documentation

Each language directory has a `*_benchmark.md` file. The adversarial directory has `adversarial_benchmark.md`. Improvements to scoring scripts, category descriptions, or pattern documentation are welcome.

## Code Style

- **Go:** Standard `gofmt` formatting. Package `testcode`. Use `shared.go` helpers.
- **Rust:** Standard `rustfmt` formatting. Module structure per benchmark doc.
- **Bash:** ShellCheck-clean. POSIX-compatible where possible.
- **Ruby:** Standard Ruby style. Use `require_relative 'shared'`. Annotation format uses `#` comments: `# vuln-code-snippet start KEY`. Prefer idiomatic patterns: string interpolation for TP sqli, array form of `system()` for TN cmdi. Backticks always go through shell (no safe form). `YAML.load` is TP; `YAML.safe_load` is TN. `Marshal.load` is always TP -- TN uses `JSON.parse` instead.
- **Adversarial:** Any language (JS/Python/Go). Code should look like realistic production code -- the concealment technique itself provides the test value, not the surrounding code structure.

## Validation

Before submitting, run the validation script to ensure CSV/file consistency:

```bash
python scripts/validate_go.py           # Go benchmark
python scripts/validate_rust.py         # Rust benchmark
python scripts/validate_bash.py         # Bash benchmark
python scripts/validate_php.py          # PHP benchmark
python scripts/validate_ruby.py         # Ruby benchmark
python scripts/validate_adversarial.py  # Adversarial benchmark (L1-L5 fidelity)
```

The validator checks:
- Every CSV key has a matching test file / annotation
- Every test file / annotation has a matching CSV entry
- No duplicate keys
- Per-category TP/TN balance summary

The adversarial validator runs 5 fidelity levels (L1-L5): structural integrity, roundtrip fidelity, schema validation, semantic fidelity (vuln-line/safe-line correctness), and scoring pipeline readiness.

## Scoring Your Tool

To score a SAST tool against the benchmark:

```bash
# Export SARIF from your tool
your-tool scan go/testcode/ --output results.sarif

# Score
python scripts/score_sarif.py results.sarif go/expectedresults-0.3.2.csv
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
