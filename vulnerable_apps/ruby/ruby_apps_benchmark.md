# Ruby Reference App Benchmarks

## Overview

3 intentionally vulnerable Ruby applications with 62 ground truth entries. Each app uses a different framework to test framework-specific source detection and taint tracking.

## Apps

| App | Framework | Files | Entries | Primary Patterns |
|-----|-----------|-------|---------|------------------|
| rack_app | Raw Ruby/Rack | 7 | 24 | SQLi, XSS, SSTI, path traversal, deserialization, file upload |
| rails_api | Rails 7 | 5 | 26 | ActiveRecord injection, mass assignment, CSRF, SSTI, file upload |
| sinatra_app | Sinatra 3 | 3 | 12 | SQLi, XSS, SSRF, command injection, open redirect, mass assignment |

## Ground Truth Format

`expectedresults-0.1.0.csv` uses the same format as the main benchmark CSVs:

```csv
# test name,category,real vulnerability,CWE
rk_hardcoded_db_pass,hardcodedcreds,true,798
rk_hardcoded_env,hardcodedcreds,false,798
```

Test names map to inline `vuln-code-snippet` annotations in the source files:

```ruby
# vuln-code-snippet start rk_sqli_concat
  rows = db.execute("SELECT * FROM users WHERE username = '" + username + "'")  # vuln-code-snippet vuln-line rk_sqli_concat
# vuln-code-snippet end rk_sqli_concat
```

## Categories

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 4 | 4 | 8 |
| xss | 79 | 3 | 3 | 6 |
| cmdi | 78 | 2 | 2 | 4 |
| pathtraver | 22 | 2 | 2 | 4 |
| redirect | 601 | 2 | 2 | 4 |
| hardcodedcreds | 798 | 2 | 2 | 4 |
| securecookie | 614 | 2 | 2 | 4 |
| massassign | 915 | 2 | 2 | 4 |
| ssti | 1336 | 2 | 2 | 4 |
| deserial | 502 | 2 | 2 | 4 |
| fileupload | 434 | 2 | 2 | 4 |
| csrf | 352 | 2 | 2 | 4 |
| weakhash | 328 | 1 | 1 | 2 |
| weakrand | 330 | 1 | 1 | 2 |
| ssrf | 918 | 1 | 1 | 2 |
| headerinj | 113 | 1 | 1 | 2 |
| **Total** | | **31** | **31** | **62** |

## Key Patterns

- **Framework-specific sources**: `params[:name]` (Rails), `request.params['q']` (Sinatra), `env['QUERY_STRING']` (Rack)
- **ORM vs raw SQL**: ActiveRecord `.where(string)` vs `.where("col = ?", val)`
- **Mass assignment**: `params.permit!` vs `params.permit(:name, :email)`
- **Template injection**: `ERB.new(user_input).result` vs `ERB.new(File.read('template.erb')).result`

## How to Score

```bash
# 1. Run your SAST tool on an app
your-tool scan vulnerable_apps/ruby/rack_app/ --format sarif -o results.sarif

# 2. Match findings to ground truth annotation keys
# A finding at a vuln-code-snippet line range is a detection
```

See [../README.md](../README.md) for the full scoring methodology.

## Annotation Key Prefixes

| Prefix | App |
|--------|-----|
| `rk_` | rack_app |
| `ra_` | rails_api |
| `si_` | sinatra_app |
