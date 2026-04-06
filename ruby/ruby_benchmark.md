# Ruby SAST Benchmark v0.2.0

## Purpose

The first public OWASP-style SAST benchmark for Ruby. No equivalent exists from OWASP, NIST, or any other organization in the standard OWASP Benchmark format. This benchmark provides ground truth for measuring Ruby static analysis tool accuracy.

## Test Case Inventory

27 CWE categories, 573 test cases (285 TP / 288 TN), ~50/50 balance.

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
| Raw Ruby/Rack | 2.7 - 3.3 | testcode + rack_app |
| Rails | 7.x | rails_api |
| Sinatra | 3.x | sinatra_app |

## Applications

| App | Framework | Test Cases | Description |
|-----|-----------|------------|-------------|
| rack_app | Raw Ruby/Rack | 24 | Raw Rack app with auth, posts, admin, uploads |
| rails_api | Rails 7 | 26 | REST API with ActiveRecord, strong params, CSRF |
| sinatra_app | Sinatra 3 | 12 | Route-based app with SQL, XSS, SSRF patterns |

## Annotation Format

Test cases are marked with `vuln-code-snippet` comments (same format as PHP, Rust, and Bash benchmarks):

```ruby
# vuln-code-snippet start ruby_sqli_ar_concat
def get_user(params)
  id = params[:id]
  User.where("id = #{id}") # vuln-code-snippet vuln-line ruby_sqli_ar_concat
end
# vuln-code-snippet end ruby_sqli_ar_concat
```

- `vuln-code-snippet start KEY` -- Opens a test case
- `vuln-code-snippet end KEY` -- Closes a test case
- `vuln-code-snippet vuln-line KEY` -- Marks the vulnerable line (TP test cases)
- `vuln-code-snippet safe-line KEY` -- Marks the safe line (TN test cases)

## Ground Truth

`expectedresults-0.2.0.csv` -- OWASP CSV format:

```csv
# test name,category,real vulnerability,CWE
ruby_sqli_ar_concat,sqli,true,89
ruby_sqli_ar_prepared,sqli,false,89
```

## Scoring

See [SCORING.md](SCORING.md) for full methodology and tool-specific instructions.

## Validation

```bash
python scripts/validate_ruby.py
```

Runs L1-L6 fidelity checks: structural integrity, roundtrip fidelity, schema validation, semantic fidelity, scoring pipeline readiness, and SARIF integrity.

## Ruby-Unique CWEs

These 6 categories have no equivalent in Go, Rust, Bash, or PHP benchmarks (or differ meaningfully in Ruby's idioms):

1. **CWE-915 (Mass Assignment)**: `params.permit!` or model `.update(params)` without strong parameters. Rails' `attr_accessible`/`strong_parameters` system is unique to the Rails ecosystem; bypassing it exposes model attributes that should never be user-writable.

2. **CWE-94 (Dynamic Method Dispatch)**: `object.send(params[:method])` -- Ruby's `send` and `public_send` allow calling arbitrary methods by name at runtime. Combined with `method_missing` and `define_method`, taint can flow through dynamically dispatched calls in ways static analyzers rarely track.

3. **CWE-1336 (SSTI via ERB)**: `ERB.new(user_input).result(binding)` -- Ruby's standard library ERB evaluates arbitrary Ruby expressions inside `<%= %>` tags. Unlike PHP's Twig or Python's Jinja2, ERB is bundled with the standard library and is trivially misused as a template engine fed user-controlled strings.

4. **CWE-470 (Unsafe Reflection)**: `Object.const_get(params[:klass]).new` -- Ruby's open object model allows instantiating arbitrary classes from strings. Combined with `autoload` and Zeitwerk, this can instantiate any class reachable in the load path.

5. **CWE-98 (File Inclusion via load/require)**: `load(params[:path])` or `require(user_path)` -- Ruby's `load` executes arbitrary `.rb` files and `require` loads gems or files by path. Unlike PHP's `include`, Ruby's load/require is less commonly flagged by SAST tools despite identical risk.

6. **CWE-1333 (ReDoS)**: Catastrophic backtracking in Ruby regular expressions. Ruby uses a backtracking NFA engine; patterns like `/(a+)+$/` applied to adversarial input cause exponential time. Unique to this benchmark as a standalone testable category.
