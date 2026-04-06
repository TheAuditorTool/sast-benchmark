# Go Benchmark Changelog

## v0.4.0 (2026-04-07)

### Category Expansion to 10/10 Minimum (+132 tests across 17 categories)
- All 24 existing categories now have minimum 10 TP + 10 TN tests
- weakcipher CWE-327 (00537-00540): AES-CFB without MAC, null cipher (base64), AES-256-GCM, XChaCha20-Poly1305
- securecookie CWE-614 (00541-00544): SameSite=None without Secure, missing flags, __Secure- prefix, securecookie rotation
- redirect CWE-601 (00545-00548): javascript: scheme, data: URI, url.Parse+Host allowlist, path-only struct
- hardcodedcreds CWE-798 (00549-00556): OAuth secret, SMTP password, webhook secret, encryption key vs Vault, KMS, env, config file
- authnfailure CWE-287 (00557-00564): bcrypt logic inversion, timing-unsafe ==, Basic auth ==, refresh no expiry vs correct bcrypt, ConstantTimeCompare, argon2id, expiry check
- trustbound CWE-501 (00565-00572): tenant/cookie/header/redirect to session vs JWT claims, allowlist, introspection, URL validation
- ldapi CWE-90 (00573-00580): OR filter, modify DN, scope injection, department filter vs EscapeFilter, DN from search, scope enum, attribute allowlist
- deserial CWE-502 (00581-00588): gob cookie, xml interface{}, reflect.New, yaml map vs MaxBytesReader+DisallowUnknownFields, typed struct, LimitReader, validation
- codeinj CWE-94 (00589-00596): user delimiters, ParseGlob, FuncMap os.ReadFile, side-effect method vs html/template, safe FuncMap, embed.FS, pre-compiled
- loginjection CWE-117 (00597-00603): Fprintf log.Writer, zerolog Msg concat, User-Agent, slog concat vs zerolog structured, %q verb, LogValuer
- nosql CWE-943 (00604-00610): raw JSON filter, aggregate pipeline, $set field name, DeleteMany vs bson.D $eq, typed struct, constant pipeline
- authzfailure CWE-862 (00611-00617): bulk export, horizontal IDOR, admin no role check vs JWT scope, SQL JOIN ownership, ABAC policy, role middleware
- csrf CWE-352 (00618-00624): multipart no token, JSON credentials:include, DELETE no CSRF vs gorilla/csrf, HMAC token, SameSite+Origin, Referer
- fileupload CWE-434 (00625-00636): PUT raw body, double extension, zip-slip, user directory, batch upload, file overwrite vs DetectContentType, MaxBytesReader, combo check, MkdirTemp, re-encoding, zip-slip protection
- inputval CWE-20 (00637-00648): integer overflow, NaN/Inf, header length, type assertion, nil map, negative capacity vs DisallowUnknownFields, range check, enum allowlist, length check, IsNaN/IsInf, comma-ok
- tlsverify CWE-295 (00649-00658): custom DialTLS, empty ServerName, partial VerifyPeerCertificate, VerifyConnection nil, websocket skip vs SystemCertPool, mTLS, cert pinning, TLS 1.3 only, CN check
- race_condition CWE-362 (00659-00668): balance TOCTOU, file read-modify-write, bool flag, RLock for write, lazy init vs RWMutex, atomic.Value, channel semaphore, SELECT FOR UPDATE, sync.Pool

### New CWE Category: Information Disclosure CWE-200 (+20 tests)
- **infodisclosure** (00669-00688): 10 TP + 10 TN
- TP: debug.Stack(), %+v wrapped errors, public pprof, runtime.Stack() recovery, DB err.Error(), os.Environ(), header reflection, config secrets, Server header, internal IP in error
- TN: generic error + slog, custom error type, pprof behind auth, request ID only, logged internally, health status only, admin role check, json:"-" tags, Server header omitted, wrapped internal + generic external

### Infrastructure
- New CSV: expectedresults-0.4.0.csv
- validate_go.py: added CWE 200, infodisclosure category
- go.mod: added gorilla/csrf v1.7.2

### Final State
- 686 test cases (was 534)
- 25 CWE categories (was 24)
- 343/343 TP/TN balance (exact 50/50)
- All 25 categories have minimum 10 TP + 10 TN tests

## v0.3.2 (2026-03-23)

### Thin Category Expansion (+32 tests)
- Expanded 8 categories from <10 tests to 12+ minimum
- trustbound CWE-501 (00479-00482): gorilla/sessions user role injection, context.WithValue
- ldapi CWE-90 (00483-00486): LDAP filter concatenation, BaseDN injection, EscapeFilter
- deserial CWE-502 (00487-00490): gob.Decode to interface{}, typed struct decode
- codeinj CWE-94 (00491-00494): template FuncMap with exec, plugin.Open with user path
- loginjection CWE-117 (00495-00497, 00533): log.Printf newline injection, zerolog structured
- nosql CWE-943 (00498-00500, 00534): operator injection via user field name, regex pattern
- csrf CWE-352 (00501-00503, 00535): PUT/POST without CSRF token, SameSite+token validation
- authzfailure CWE-862 (00504-00506, 00536): IDOR, X-Role header trust, DB role lookup

### New OWASP Top 25 Categories (+26 tests)
- **CWE-362 Race Condition** (00507-00516): concurrent map write, TOCTOU, closure capture race, shared counter vs sync.Mutex, atomic, channels, sync.Once
- **CWE-434 Unrestricted File Upload** (00517-00524): original filename, Content-Type spoofing, extension blocklist vs UUID rename, magic bytes, size limit, out-of-root storage
- **CWE-20 Improper Input Validation** (00525-00532): array bounds, ReDoS regex, URL scheme, negative quantity vs bounds check, net/mail, allowlist, range validation

### Quality
- All 58 new test files use benchmarkTestNNNNN prefix for package-level identifiers
- Zero vulnerability hints in any new file
- go.mod updated with zerolog dependency

### Final State
- 534 test cases (was 476)
- 24 CWE categories (was 21)
- 267/267 TP/TN balance (exact 50/50)
- Zero thin categories (all 24 have 8+ tests, 20 have 12+)
- OWASP Top 25 coverage: 14/25 (was 11/25)

## v0.3.1 (2026-03-22)

### OWASP Feedback Response

This release addresses feedback from the OWASP Foundation review.

### Tool-Agnostic SARIF Scoring
- Created `scripts/score_sarif.py` -- universal SARIF 2.1.0 scorer (stdlib-only Python, zero dependencies)
- Created `scripts/convert_theauditor.py` -- bridge from TheAuditor DB to standard SARIF
- Created `go/SCORING.md` -- full scoring documentation with commands for Semgrep, Gosec, CodeQL, SonarQube
- Scoring supports both flat aggregate and category-averaged (OWASP standard) methods
- Replaced hardcoded TheAuditor-specific scoring script in go_benchmark.md with tool-agnostic instructions

### Classification Fixes
- Fixed BenchmarkTest00209 (securecookie=true): Changed `Secure: true, HttpOnly: false` to `Secure: false, HttpOnly: true` so the code actually demonstrates CWE-614 (missing Secure attribute) rather than CWE-1004
- Fixed BenchmarkTest00340 (sqli=true): Split `BenchSvcProcessAll` (which had sqli+cmdi+pathtraver sinks) into `BenchSvcProcessSQL` (sqli only) so the test is single-concern
- Reclassified BenchmarkTest00354 (nosql): true->false. `$ne` with user string is standard comparison, not structural injection
- Reclassified BenchmarkTest00346 (loginjection): true->false. `slog.Info("request", "input", data)` uses structured key-value logging; values are escaped by slog handlers
- Added BenchmarkTest00425 (nosql=true): `$where` JavaScript injection to compensate reclassification
- Added BenchmarkTest00426 (loginjection=true): `slog.Info("msg: " + data)` message concatenation to compensate reclassification

### New CWE Categories (+50 tests)
- **CWE-798 Hardcoded Credentials** (00429-00440): Hardcoded DSN, JWT secret, API key, SSH key, admin password vs environment variables, config files, file-based keys
- **CWE-287 Authentication Failures** (00441-00452): JWT ParseUnverified, empty HMAC secret, algorithm confusion, error ignored vs algorithm assertion, claims validation, session DB lookup
- **CWE-862 Authorization Failures** (00453-00461): IDOR, missing role check, client-supplied role header vs ownership check, DB-enforced authz, JWT claims
- **CWE-352 CSRF** (00462-00470): State-changing POST without CSRF token vs form field validation, X-CSRF-Token header, double-submit cookie
- **CWE-94 Code/Template Injection** (00471-00478): User-controlled text/template string, FuncMap with exec, template file path injection vs hardcoded templates, allowlisted paths

### Quality
- All 50 new test files personally verified (read every file, fixed naming collisions, removed incidental SQLi)
- 7 package-level identifier naming collisions fixed to prevent Go compile errors
- Fixed incidental SQLi in BenchmarkTest00438 (hardcodedcreds=false test had `"SELECT...FROM "+table`)
- go.mod updated with `golang-jwt/jwt/v5` and `gorilla/csrf` dependencies

### Scoring Methodology
- Added category-averaged scoring (OWASP standard): each category weighted equally regardless of test count
- TheAuditor baseline: +24.9% category-averaged (vs +29.2% flat)

### Hint Removal (OWASP Gold Standard Compliance)

Stripped all vulnerability classification hints from apps/ source code. The ground_truth.csv in each app is now the sole oracle -- matching OWASP Java/Python benchmark design and the Rust benchmark's v0.3.1 hint removal.

**apps/ (69 files across 5 apps):**
- Removed ~838 whole-line hint comments (`// TAINT SOURCE:`, `// TAINT SINK:`, `// VULNERABLE:`, `// SECURE:`, etc.)
- Stripped ~188 inline hint comments from code lines (kept code, removed trailing classification comments)
- Stripped `# VULN:` comments from shell scripts (4 files), YAML config (1 file), and `<!-- VULN: -->` from HTML templates (2 files)
- Includes .proto file hint stripping in grpc_users
- Neutralized function doc comments from `// FuncName - VULNERABLE: desc` to `// FuncName does X`

**Validation:**
```
grep -rn "TAINT|VULNERABLE|VULN:|SECURE:|SOURCE:|SINK" go/apps/ -- 0 results
grep -rn "TAINT|VULNERABLE|VULN:|SECURE:|SOURCE:|SINK" go/testcode/ -- 0 results (was already clean)
```

### Final State
- 476 test cases (was 424)
- 21 CWE categories (was 16)
- 238/238 TP/TN balance (exact 50/50)
- Tool-agnostic SARIF-based scoring

## v0.3 (2026-03-20)

### v0.3-rc3: Personal Read Verification
- Manually read 24 additional test files focusing on edge cases and debatable classifications
- Fixed BenchmarkTest00146 (ssrf=false): IP validation was applied to `host` param but `http.Get` fetched separate `targetURL` param. Validation now applies to the URL actually being fetched via `url.Parse(targetURL).Hostname()`.
- Fixed BenchmarkTest00398 (cmdi=true): Used `exec.Command("cat", file)` with hardcoded command -- same pattern as tests 054/066 which are correctly classified as cmdi=false. Changed to `exec.Command("sh", "-c", "cat "+file)` so shell metacharacters in `file` CAN execute injected commands, making the cmdi=true classification correct.

### v0.3-rc2: Deep Verification Pass (2026-03-19)
- 3 audit agents verified all 314 agent-written test files against CSV classifications
- 99.4% accuracy (312/314 correct)
- Fixed BenchmarkTest00118 (xss): reclassified true->false. Go's encoding/json escapes HTML by default (`<` -> `\u003c`). Even with Content-Type text/html, json.Encoder output cannot inject HTML tags.
- Fixed BenchmarkTest00255 (deserial): removed user-controlled sessionID from file path. The gob.Decode to typed struct was safe, but os.Open path was built from query param (path traversal). Changed to hardcoded internal path.
- TP/TN split improved from 213/211 to 212/212 (perfect 50/50)
- Updated all documentation to reflect corrected numbers

### v0.3-rc1: Scale Expansion (2026-03-19)
- GORM `db.Raw()` vs `db.Where()` discrimination (375-376)
- sqlx `NamedExec` vs `Get` with `$1` placeholder (377-378)
- Error path taint: `err.Error()` reused in query (379-380)
- Batch VALUES builder vs prepared statement loop (381-382)
- `text/template` for SQL query construction (383-384)
- Deferred function taint propagation (385-386)
- Multipart `header.Filename` in query (387-388)
- CTE/WITH clause injection (389-390)
- GORM `Joins()` with raw condition (391-392)
- Dynamic column selection from user input (393-394)
- `syscall.Exec` and `os.StartProcess` (395-397)
- TOCTOU pattern: stat then exec (398-399)
- PATH environment manipulation (400-401)
- Pipe construction with user input (402-403)
- `text/template` for shell commands (404-405)
- URL path segment as tool name (406)
- Zip slip via `archive/zip` (407-408)
- `filepath.EvalSymlinks` defense (409)
- `os.MkdirAll`, `os.Rename`, `os.Symlink` with user paths (410-416)
- WebSocket message echo (417-418)
- Server-Sent Events (SSE) injection (419-420)
- `html/template` FuncMap bypass (421-422)
- Content-Disposition header injection (423-424)

### Missing CWE Categories (+32 tests)
- Log Injection CWE-117 (343-350): `log.Printf`, `slog.Info`, file-based logging
- NoSQL Injection CWE-943 (351-358): MongoDB `$where`, `$regex`, `$ne`, `bson.M` operator injection
- LDAP Injection CWE-90 (359-366): `go-ldap/ldap` filter concatenation vs `EscapeFilter`
- Trust Boundary CWE-501 (367-374): `gorilla/sessions` user input in session values

### Cross-File Multi-Hop Flows (+17 tests + benchmark_services.go)
- 2-hop: handler -> BenchSvcQueryUser/ExecCmd/ReadPath (326-332)
- 3-hop: handler -> BenchSvcTransform -> BenchSvcBuildQuery (333-335)
- Struct field propagation across files (336-337)
- Return value taint chain (338-339)
- Multiple sinks from one service call (340)
- Method receiver pattern on BenchSvcStore (341-342)

### Framework Source Diversity (+33 tests)
- Gin: `c.Query()`, `c.Param()` (293-297)
- Echo: `ctx.QueryParam()`, `ctx.Param()` (298-302)
- Chi: `chi.URLParam(r, "key")` (303-307)
- Fiber: `c.Query()`, `c.Params()` (308-312)
- Gorilla Mux: `mux.Vars(r)["key"]` (313-317)
- Beego: `c.GetString()`, `c.Ctx.Input.Param()` (318-321)
- gRPC-style: `req.FieldName` struct fields (322-325)

### OWASP-Style Discrimination Patterns (+36 tests)
- Dead-code conditionals: `if 7*42 > 200 { bar = "safe" }` (257-262)
- Variable overwrite: `bar = param; bar = "constant"` (263-268)
- Map key confusion: `m["a"] = param; bar = m["b"]` (269-272)
- Slice/array propagation (273-275)
- Go channel propagation (276-278)
- Interface type assertion (279-280)
- Assignment chain (281-282)
- Struct field overwrite vs propagation (283-286)
- All-constant branches (287-288)
- bytes.Buffer concat, closure capture, int round-trip, Sprintf propagation (289-292)

### App Integration
- Ground truth CSVs for 5 reference apps (394 classified functions total)
- multi-api: 114 entries
- calorie-tracker: 137 entries
- go_notifications: 87 entries
- beego_admin: 32 entries
- grpc_users: 24 entries

### Quality
- Removed init() hack in benchmark_test_00074.go
- Zero vulnerability hint comments verified across all 424 test files
- Zero duplicate type/function names
- Zero import conflicts

## v0.1 (2026-03-19)

- Initial benchmark: 256 test cases across 12 CWE categories
- Primary framework: net/http standard library
- 6 reference apps copied from source projects
- CSV answer key, scoring script, documentation
