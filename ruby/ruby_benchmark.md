# Ruby SAST Benchmark v0.3.2

## Purpose

The first public OWASP-style SAST benchmark for Ruby. No equivalent exists from OWASP, NIST, or any other organization in the standard OWASP Benchmark format. This benchmark provides ground truth for measuring Ruby static analysis tool accuracy.

## Test Case Inventory

27 CWE categories, 1,288 test cases (644 TP / 644 TN), exact 50/50 balance.

### Tier 1: Core (High SAST Detectability)

| Category | CWE | Description | Ruby-Specific |
|----------|-----|-------------|---------------|
| sqli | 89 | SQL Injection | ActiveRecord .where("#{x}"), find_by_sql, Sequel .lit |
| xss | 79 | Cross-Site Scripting | .html_safe, raw(), ERB <%== %>, render inline: |
| cmdi | 78 | Command Injection | backticks, system(), exec(), %x{}, IO.popen, Open3 |
| pathtraver | 22 | Path Traversal | File.read, File.open, send_file, IO.read |
| codeinj | 94 | Code Injection | eval(), instance_eval, class_eval, binding.eval |
| deserial | 502 | Insecure Deserialization | Marshal.load, YAML.load, YAML.unsafe_load |
| ssrf | 918 | Server-Side Request Forgery | Net::HTTP, open-uri, HTTParty, Faraday |
| fileinclusion | 98 | File Inclusion | load/require with user-controlled path |

### Tier 2: Ruby-Specific (Differentiators)

| Category | CWE | Description | Ruby-Specific |
|----------|-----|-------------|---------------|
| massassign | 915 | Mass Assignment | params.permit!, missing strong_parameters |
| unsafereflect | 470 | Unsafe Reflection | const_get, send, public_send from string |
| ssti | 1336 | Server-Side Template Injection | ERB.new(user_input).result, Haml from tainted |
| dynmethod | 94 | Dynamic Method Dispatch | define_method forwarding taint, method_missing |
| redirect | 601 | Open Redirect | redirect_to params[:url] |
| xxe | 611 | XML External Entity | Nokogiri::XML without NONET, REXML |

### Tier 3: Authentication & Authorization

| Category | CWE | Description | Ruby-Specific |
|----------|-----|-------------|---------------|
| authnfailure | 287 | Improper Authentication | JWT unverified/none-alg, skip_before_action, session fixation, Devise bypass |
| authzfailure | 862 | Missing Authorization | IDOR, missing Pundit/CanCanCan, tenant isolation, unscoped queries |

### Tier 4: Standard Web

| Category | CWE | Description |
|----------|-----|-------------|
| csrf | 352 | Cross-Site Request Forgery |
| weakhash | 328 | Weak Hash (md5/sha1 for passwords) |
| weakrand | 330 | Weak Random (rand/srand) |
| weakcipher | 327 | Weak Cipher (DES, RC4, ECB) |
| hardcodedcreds | 798 | Hardcoded Credentials |
| headerinj | 113 | HTTP Header Injection |
| ldapi | 90 | LDAP Injection |
| securecookie | 614 | Insecure Cookie (missing Secure/HttpOnly) |
| fileupload | 434 | Unrestricted File Upload |
| loginjection | 117 | Log Injection |
| regex | 1333 | ReDoS (Catastrophic Backtracking) |

## Frameworks Covered

| Framework | Version Target | Usage |
|-----------|---------------|-------|
| Raw Ruby/Rack | 2.7 - 3.3 | testcode |

## Test File Format

Each test file is a standalone Ruby script containing one function. Files contain
zero comments -- no annotations, no prose, no CWE references. This follows the
OWASP Java Benchmark gold standard (identical to Go).

```ruby
require_relative 'shared'

def get_user_profile(req)
  db = get_db_connection
  id = req.param('id')
  rows = db.execute("SELECT * FROM users WHERE id = #{id}")
  user = rows.first
  return BenchmarkResponse.bad_request('user not found') unless user
  BenchmarkResponse.json({ id: user[0], name: user[1], email: user[2] })
end
```

## Anti-Target Leakage Rules

1. **1 file = 1 test** -- every testcode file contains exactly one test case
2. **Generic filenames** -- `benchmark_test_NNNNN.rb`, no category in filename
3. **Zero comments** -- no annotations, no prose, no trailing explanations
4. **Opaque CSV keys** -- `BenchmarkTestNNNNN`, no category encoding
5. **Domain-descriptive function names** -- never category-prefixed (`xss_*`, `ssti_*`, etc.)
6. **No CWE references** -- no CWE numbers anywhere in test files

These rules ensure SAST tools must rely on actual dataflow/AST analysis rather
than text-matching filenames, comments, or annotations.

## Ground Truth

`expectedresults-0.3.2.csv` -- OWASP CSV format:

```csv
# test name,category,real vulnerability,CWE
BenchmarkTest00001,ssrf,true,918
BenchmarkTest00002,weakcipher,true,327
```

## Scoring

Filename-based matching with CWE awareness (same as Go/Java):

```bash
python scripts/score_sarif.py results.sarif ruby/expectedresults-0.3.2.csv
```

A test case is detected if a SARIF finding references the test file AND the
finding's ruleId CWE matches the expected CWE. See [SCORING.md](SCORING.md).

## Validation

```bash
python scripts/validate_ruby.py
```

Runs structural integrity, file existence, CSV consistency, and TP/TN balance checks.

## Ruby-Unique CWEs

These 6 categories have no equivalent in Go, Rust, Bash, or PHP benchmarks (or differ meaningfully in Ruby's idioms):

1. **CWE-915 (Mass Assignment)**: `params.permit!` or model `.update(params)` without strong parameters. Rails' `attr_accessible`/`strong_parameters` system is unique to the Rails ecosystem; bypassing it exposes model attributes that should never be user-writable.

2. **CWE-94 (Dynamic Method Dispatch)**: `object.send(params[:method])` -- Ruby's `send` and `public_send` allow calling arbitrary methods by name at runtime. Combined with `method_missing` and `define_method`, taint can flow through dynamically dispatched calls in ways static analyzers rarely track.

3. **CWE-1336 (SSTI via ERB)**: `ERB.new(user_input).result(binding)` -- Ruby's standard library ERB evaluates arbitrary Ruby expressions inside `<%= %>` tags. Unlike PHP's Twig or Python's Jinja2, ERB is bundled with the standard library and is trivially misused as a template engine fed user-controlled strings.

4. **CWE-470 (Unsafe Reflection)**: `Object.const_get(params[:klass]).new` -- Ruby's open object model allows instantiating arbitrary classes from strings. Combined with `autoload` and Zeitwerk, this can instantiate any class reachable in the load path.

5. **CWE-98 (File Inclusion via load/require)**: `load(params[:path])` or `require(user_path)` -- Ruby's `load` executes arbitrary `.rb` files and `require` loads gems or files by path. Unlike PHP's `include`, Ruby's load/require is less commonly flagged by SAST tools despite identical risk.

6. **CWE-1333 (ReDoS)**: Catastrophic backtracking in Ruby regular expressions. Ruby uses a backtracking NFA engine; patterns like `/(a+)+$/` applied to adversarial input cause exponential time. Unique to this benchmark as a standalone testable category.
