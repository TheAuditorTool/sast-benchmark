# Go SAST Benchmark v0.3.1

## Overview

OWASP-style SAST benchmark for Go. **476 test cases** across **21 CWE categories** with 50/50 vulnerable/safe split (238/238). Plus 5 reference apps with 395 classified functions.

**Design principles**:
- Test cases written from security knowledge, NOT from knowledge of any specific SAST engine's detection capabilities
- No vulnerability hints in source code -- the CSV answer key is the ONLY ground truth
- 50/50 TP/TN balance prevents classifier gaming (flag-everything scores 0%, not 100%)
- Category-averaged scoring prevents large categories from dominating small ones
- Tool-agnostic SARIF-based scoring -- any SAST tool can be scored

**Audit status**: All 476 test files verified 2026-03-22. Zero vulnerability hints. Zero duplicate types/functions.

---

## Structure

```
gorustbash_benchmark/go/
  expectedresults-0.1.csv     # Answer key: test,category,vulnerable,CWE
  go_benchmark.md             # This file
  SCORING.md                  # Full scoring methodology and tool instructions
  CHANGELOG.md                # Every change documented
  testcode/                   # 476 benchmark test files + shared.go + benchmark_services.go
  apps/                       # 5 reference apps with ground_truth.csv each
  cmd/main.go                 # Entry point
  go.mod
```

---

## Categories

| # | Category | CWE | Vuln | Safe | Total |
|---|----------|-----|------|------|-------|
| 1 | sqli | 89 | 65 | 58 | 123 |
| 2 | cmdi | 78 | 30 | 30 | 60 |
| 3 | pathtraver | 22 | 25 | 30 | 55 |
| 4 | xss | 79 | 13 | 15 | 28 |
| 5 | ssrf | 918 | 10 | 10 | 20 |
| 6 | weakrand | 330 | 10 | 10 | 20 |
| 7 | weakhash | 328 | 10 | 10 | 20 |
| 8 | weakcipher | 327 | 8 | 8 | 16 |
| 9 | securecookie | 614 | 8 | 8 | 16 |
| 10 | redirect | 601 | 8 | 8 | 16 |
| 11 | hardcodedcreds | 798 | 6 | 6 | 12 |
| 12 | authnfailure | 287 | 6 | 6 | 12 |
| 13 | tlsverify | 295 | 5 | 5 | 10 |
| 14 | loginjection | 117 | 4 | 5 | 9 |
| 15 | nosql | 943 | 4 | 5 | 9 |
| 16 | authzfailure | 862 | 5 | 4 | 9 |
| 17 | csrf | 352 | 5 | 4 | 9 |
| 18 | codeinj | 94 | 4 | 4 | 8 |
| 19 | ldapi | 90 | 4 | 4 | 8 |
| 20 | trustbound | 501 | 4 | 4 | 8 |
| 21 | deserial | 502 | 4 | 4 | 8 |
| | **TOTAL** | | **238** | **238** | **476** |

---

## Current Scorecard

**Baseline: Not yet established. Run `aud full --offline` and score.**

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
(pending first run)
----------------------------------------------------------------------
OVERALL
```

---

## Test Case Registry

### SQL Injection (CWE-89) -- Tests 00001-00050

**VULNERABLE patterns (25 tests):**

| Pattern | Tests | Source | Sink |
|---------|-------|--------|------|
| fmt.Sprintf to db.Query | 1, 13, 42 | r.URL.Query, r.PostFormValue | db.Query, db.Exec |
| String concat (`+`) | 3 | r.URL.Query | db.QueryRow |
| strings.Builder | 5 | r.URL.Query | db.Query |
| strings.Join + Sprintf (table/col) | 7 | r.Form, r.FormValue | db.Query |
| Cross-function hop | 9 | r.URL.Query | helper -> db.Exec |
| JSON body struct fields | 11 | json.Decode(r.Body) | db.Exec |
| Header source | 15, 48 | r.Header.Get | db.Exec |
| Cookie source | 17 | r.Cookie().Value | db.Query |
| Map iteration (dynamic cols) | 19 | json.Decode to map | db.Exec in loop |
| Switch branches all concat | 20 | r.URL.Query (two params) | db.Query |
| Fake sanitizer (passthrough) | 23 | r.URL.Query -> passthrough func | db.Query |
| Incomplete escaping (Replace) | 22 | r.URL.Query -> strings.Replace | db.Query |
| Raw body to SQL | 25 | io.ReadAll(r.Body) | db.Exec |
| Loop batch concat | 27 | r.Form["item"] array | db.Exec per item |
| Second-order (DB value reused) | 30 | db.QueryRow result -> Sprintf | db.Query |
| Transaction with concat | 33 | r.URL.Query | tx.Exec |
| ORDER BY injection | 36 | r.URL.Query (sort, order) | db.Query |
| Context value taint | 38 | r.URL.Query -> context.WithValue -> retrieve | db.Query |
| URL path slice | 40 | r.URL.Path[len("/users/"):] | db.Query |
| Multi-param Sprintf | 42 | r.FormValue (3 params) | db.Query |
| Goroutine closure | 44 | r.URL.Query -> go func | db.Exec |
| Interface{} type assertion | 46 | json.Decode to interface{} -> cast | db.Query |
| Multi-function chain | 50 | r.URL.Query -> buildWhere -> buildQuery | db.Query |

**SAFE patterns (25 tests):**

| Pattern | Tests | Why Safe |
|---------|-------|----------|
| Parameterized `?` | 2, 6, 14, 16, 18, 39, 41, 49 | Placeholder prevents injection |
| Prepared statement | 4, 28, 34 | db.Prepare + stmt.Exec |
| strconv.Atoi validation | 10 | Integer conversion rejects non-numeric |
| JSON body + parameterized | 12 | Struct binding + `?` placeholders |
| Conditional + parameterized | 21 | Switch branches all use `?` |
| Dead variable (overwritten) | 24 | Input assigned, constant used instead |
| Regex + parameterized | 26 | ^[a-zA-Z0-9]+$ then `?` |
| sha256 hash before use | 29 | Input hashed, hash used parameterized |
| Allowlisted table names | 31 | map[string]bool lookup |
| ParseUint + %d format | 32 | Numeric-only after validation |
| Input ignored (constant query) | 35 | User input read but never used in query |
| Allowlisted sort columns | 37 | map[string]string lookup, default fallback |
| Multiple params all safe | 39 | All 3 params via `?` |
| Exec with args | 41 | db.Exec("...?", id) |
| Constant-only concat | 43 | "SELECT " + "id, name" -- all string literals |
| Goroutine + parameterized | 45 | go func with `?` placeholder |
| UUID validation | 47 | uuid.Parse validates format first |
| QueryRow + parameterized | 49 | db.QueryRow("...?", input) |
| No user input at all | 8 | Static query, no request params used |

### Command Injection (CWE-78) -- Tests 00051-00080

**VULNERABLE patterns (13 tests):**

| Pattern | Tests | Mechanism |
|---------|-------|-----------|
| User-controlled command name | 51 | exec.Command(userInput, args) |
| Shell concat (sh -c) | 53, 55, 65 | exec.Command("sh", "-c", "cmd "+userInput) |
| Shell via bash | 61, 73 | exec.Command("bash", "-c", shellCmd) |
| JSON body to shell | 57 | input.Command -> exec.Command("sh", "-c", ...) |
| CommandContext with user cmd | 59 | exec.CommandContext(ctx, userInput, args...) |
| LookPath with user input | 63 | exec.LookPath(userInput) then execute |
| Form array as cmd+args | 70 | exec.Command(args[0], args[1:]...) |
| Cross-function shell | 75 | helper builds shell string |
| Header to shell | 78 | r.Header.Get -> exec.Command("sh", "-c", ...) |
| Env var manipulation | 68 | os.Setenv with user value |

**SAFE patterns (17 tests):**

| Pattern | Tests | Why Safe |
|---------|-------|----------|
| Hardcoded command, no user input | 52, 64, 67, 76, 79 | Command + args all constants |
| Hardcoded cmd, user in args (no shell) | 54, 60, 72, 77, 80 | exec.Command bypasses shell parsing |
| net.ParseIP validation | 56 | Only valid IPs pass |
| Regex hostname validation | 58 | ^[a-zA-Z0-9.-]+$ |
| Allowlisted commands | 62 | map[string]bool lookup |
| grep with user pattern, hardcoded file | 66 | No shell, user controls search only |
| Dead code (never Run()) | 69 | Command built but never executed |
| Input only in response | 71 | User input echoed, never to exec |
| Input ignored | 74 | User input read but not used |

### Path Traversal (CWE-22) -- Tests 00081-00110

**VULNERABLE patterns (16 tests):**

| Pattern | Tests | Mechanism |
|---------|-------|-----------|
| Direct os.ReadFile(userInput) | 81, 108 | No path validation |
| filepath.Join (doesn't prevent ..) | 83, 89, 95, 100, 105 | Join preserves ../ components |
| Direct os.Create/WriteFile | 85, 87 | User controls write destination |
| URL path slice to os.Open | 91 | r.URL.Path[len("/files/"):] |
| os.ReadDir(userInput) | 93 | Directory listing exposure |
| Multipart header.Filename | 97 | Upload filename to os.Create |
| String concat path | 102, 104 | "dir/" + userInput |
| JSON body output path | 107 | filepath.Join(input.OutputDir, ...) |
| Both src+dst from user | 110 | Read + write both user-controlled |

**SAFE patterns (14 tests):**

| Pattern | Tests | Why Safe |
|---------|-------|----------|
| filepath.Clean + HasPrefix | 82, 99, 109 | Validates path stays under base dir |
| Hardcoded path | 84, 103 | No user input in path |
| filepath.Base (strips dirs) | 86, 96 | Only filename, no directory traversal |
| os.CreateTemp (OS name) | 88 | System generates safe filename |
| Regex filename validation | 90 | ^[a-zA-Z0-9_.-]+$ |
| Integer-based path | 92 | fmt.Sprintf(".../%d.json", intID) |
| UUID-generated filename | 94, 98 | uuid.New().String() |
| Extension + Base check | 96 | Allowlisted extensions + Base() |
| Key-to-path map lookup | 101 | User provides key, server resolves path |
| Contains("..") check | 106 | filepath.Clean then reject if ".." present |

### XSS (CWE-79) -- Tests 00111-00130

**VULNERABLE (9):** template.HTML(111), template.JS(112), template.URL(113), template.CSS(114), fmt.Fprintf to HTML(115), raw w.Write concat(116), text/template(117), script tag injection(119), header reflection to HTML(120)

**SAFE (11):** Go json.Encoder HTML-escapes by default(118), html/template auto-escape(121), html.EscapeString(122), JSON content-type(123), html/template {{.}}(124), text/plain(125), url.QueryEscape(126), base64 encoding(127), int-only output(128), DB result not direct(129), static ServeFile(130)

### SSRF (CWE-918) -- Tests 00131-00150

**VULNERABLE (10):** http.Get(userURL)(131), http.Post(userURL)(132), http.NewRequest(userURL)(133), header webhook URL(134), JSON body callback(135), host concat(136), port concat(137), url.URL struct(138), FormValue endpoint(139), cookie as URL(140)

**SAFE (10):** hardcoded URL(141), allowlisted domain(142), config URL(143), scheme+host check(144), user in query param only(145), IP private check(146), display only(147), map lookup(148), user in body not URL(149), DNS resolution check(150)

### Weak Random (CWE-330) -- Tests 00151-00170

**VULNERABLE (10):** math/rand.Intn for token(151), math/rand.Read for key(152), seeded NewSource for reset(153), Float64 for CSRF(154), Int63 for API key(155), Shuffle for ID(156), aliased mathrand.Read(157), Uint32 for nonce(158), Perm for OTP(159), Int for salt(160)

**SAFE (10):** crypto/rand.Read(161), crypto/rand.Int(162), uuid.New(163), math/rand for display order(164), bcrypt(165), io.ReadFull crypto reader(166), hex of crypto bytes(167), base64 of crypto bytes(168), crypto/rand.Prime(169), math/rand for delay(170)

### Weak Hash (CWE-328) -- Tests 00171-00190

**VULNERABLE (10):** md5.Sum password(171), md5.New password(172), sha1.Sum integrity(173), sha1 HMAC key(174), md5 signature(175), md5 token(176), sha1 cert fingerprint(177), md5 reset token(178), sha1 API key(179), md5 auth file check(180)

**SAFE (10):** sha256(181), sha512(182), sha256 HMAC(183), bcrypt(184), sha256 content addr(185), hmac sha256(186), sha256 file checksum(187), md5 cache key(188), sha256 CSRF(189), sha256 digital sig(190)

### Weak Cipher (CWE-327) -- Tests 00191-00206

**VULNERABLE (8):** DES(191), 3DES(192), RC4(193), Blowfish(194), AES-ECB(195), XOR cipher(196), AES-CBC no auth(197), ROT13(198)

**SAFE (8):** AES-GCM(199,201,203,206), ChaCha20-Poly1305(200), TLS 1.3 config(202), AES-CTR+HMAC(204), NaCl secretbox(205)

### Secure Cookie (CWE-614) -- Tests 00207-00222

**VULNERABLE (8):** Secure:false(207), Secure omitted(208), HttpOnly:false(209), SameSiteNone(210), raw Set-Cookie header(211), raw user ID value(212), broad Domain(213), MaxAge:0 no flags(214)

**SAFE (8):** all flags strict(215), MaxAge+path+flags(216), gorilla securecookie(217), SameSiteLax+flags(218), crypto/rand session ID(219), __Host- prefix(220), AES-GCM encrypted value(221), short MaxAge+flags(222)

### Open Redirect (CWE-601) -- Tests 00223-00238

**VULNERABLE (8):** direct query param(223), MovedPermanently(224), raw Location header(225), HasPrefix("http") bypass(226), meta refresh(227), script location(228), JSON body return URL(229), protocol-relative //(230)

**SAFE (8):** hardcoded /dashboard(231), allowlisted domain(232), path-only extract(233), same-origin check(234), key-to-URL map(235), hardcoded /login(236), known paths list(237), HMAC-signed URL(238)

### TLS Verify (CWE-295) -- Tests 00239-00248

**VULNERABLE (5):** InsecureSkipVerify:true(239), Transport with skip(240), VerifyPeerCertificate always nil(241), MinVersion:0(242), skip+empty RootCAs(243)

**SAFE (5):** default Client(244), TLS 1.2 min(245), TLS 1.3 min(246), pinned CA pool(247), modern cipher suites(248)

### Deserialization (CWE-502) -- Tests 00249-00256

**VULNERABLE (4):** gob.Decode to interface{}(249), yaml.Unmarshal to interface{}(250), json + reflect method call(251), binary.Read from body(252)

**SAFE (4):** json to typed struct(253), yaml to typed struct(254), gob from internal file(255), json.Decoder to typed struct(256)

### Hardcoded Credentials (CWE-798) -- Tests 00429-00440

**VULNERABLE (6):** hardcoded DSN password(429), hardcoded JWT signing secret(430), hardcoded API key constant(431), hardcoded config struct password(432), SSH private key as string literal(433), hardcoded admin password in comparison(434)

**SAFE (6):** credential from os.Getenv(435), JWT key from file(436), API key from environment(437), password from JSON config file(438), SSH key path from env(439), hardcoded non-sensitive username constant(440)

### Authentication Failures (CWE-287) -- Tests 00441-00452

**VULNERABLE (6):** jwt.ParseUnverified(441), empty HMAC secret(442), no algorithm type assertion(443), parse error ignored(444), algorithm confusion RSA-as-HMAC(445), session cookie value used as identity without validation(446)

**SAFE (6):** explicit HMAC method assertion(447), env-sourced non-empty secret with assertion(448), explicit alg:none rejection(449), full claims validation exp+iss+sub(450), RS256 with RSA key type assertion(451), session validated against DB with expiry check(452)

### Authorization Failures (CWE-862) -- Tests 00453-00461

**VULNERABLE (5):** IDOR user_id from query param without ownership check(453), admin action without role check(454), client-supplied X-User-Role header trusted(455), delete without ownership check(456), file download without access control(457)

**SAFE (4):** ownership check authUserID vs requestedID(458), role from verified JWT claims(459), role from database lookup(460), SQL-enforced ownership DELETE...AND owner_id=?(461)

### CSRF (CWE-352) -- Tests 00462-00470

**VULNERABLE (5):** money transfer without CSRF token(462), account deletion without CSRF(463), password change without CSRF(464), settings update without CSRF(465), JSON API with cookie auth no CSRF(466)

**SAFE (4):** CSRF token in hidden form field validated against session(467), X-CSRF-Token header validated(468), double-submit cookie pattern(469), read-only GET handler(470)

### Code/Template Injection (CWE-94) -- Tests 00471-00478

**VULNERABLE (4):** user-controlled text/template string parsed and executed(471), user template accesses DBPassword/APIKey struct fields(472), template.ParseFiles with user-controlled path(473), FuncMap with exec.Command wrapper callable from user template(474)

**SAFE (4):** hardcoded template with user as data only(475), constant template string with user in .Content field(476), template from allowlisted file path map(477), pre-compiled templates selected by validated name(478)

---

## Reference App Inventory (apps/)

### 1. vulnerable-api

**Purpose:** Multi-framework API with intentional SQL injection, command injection, path traversal, XSS across 6 Go web frameworks.

**Frameworks:** gin, chi, echo, fiber, net/http, gorilla/mux

| File | Lines | Vulns | Safe |
|------|-------|-------|------|
| chi_handlers.go | 358 | 7 SQLi, 2 CmdI, 2 PathTrav | 2 (parameterized queries) |
| echo_handlers.go | 304 | 7 SQLi, 2 CmdI, 1 PathTrav | 1 (parameterized) |
| fiber_handlers.go | 395 | 8 SQLi, 2 CmdI, 2 PathTrav | 2 (parameterized, prepared) |
| gin_handlers.go | 337 | 5 SQLi, 2 CmdI, 3 PathTrav, 3 XSS | 2 (parameterized, hardcoded cmd) |
| nethttp_handlers.go | 447 | 9 SQLi, 3 CmdI, 3 PathTrav | 2 (parameterized, prepared) |
| middleware.go | 296 | 1 CORS reflection | - |
| multihop_handlers.go | 373 | Multi-hop flows (handler->service) | - |
| data_service.go | 317 | 8 SQLi sinks, 1 CmdI, 1 PathTrav | - |
| async_service.go | ~100 | Goroutine taint flows | - |

**Key patterns:** fmt.Sprintf, string concat, strings.Builder, fake sanitizer, multi-hop cross-file, struct field taint, map iteration, conditional branches, loop batch, goroutine closure.

**Annotations:** All functions annotated with TAINT SOURCE/PROPAGATION/SINK comments.

### 2. calorie-tracker

**Purpose:** Real-world calorie tracking API. Mostly safe (GORM ORM) with 2 intentional vulnerable methods.

**Framework:** gin, GORM

| File | Vulns | Notes |
|------|-------|-------|
| user_repository.go:70-74 | 1 SQLi | db.Raw(fmt.Sprintf(...)) |
| food_repository.go:55-60 | 1 SQLi | db.Raw(fmt.Sprintf(...)) |
| All other files | 0 | GORM parameterized: db.First, db.Where("?", val) |

**Key insight:** Demonstrates that ORM usage is safe EXCEPT when using Raw() with string formatting.

### 3. go_notifications

**Purpose:** Notification dispatch service with template rendering, webhooks, cross-service calls.

**Framework:** gorilla/mux

| Vuln Type | Count | Key Locations |
|-----------|-------|---------------|
| SQL Injection | 14 | gorilla_handlers.go throughout |
| Command Injection | 3 | handlers.go:290, 437 |
| Path Traversal | 3 | handlers.go:277, 321, 351 |
| SSRF | 1 | handlers.go:248 (webhook URL) |
| Template Injection | 1 | handlers.go:188 |
| Log Injection | 1 | handlers.go:421-431 |
| Info Disclosure | 1 | handlers.go:458-471 (DebugRequest) |
| Cross-Service Taint | 2 | handlers.go:521, 587 (calls beego_admin) |

**Key patterns:** mux.Vars() source, wildcard path params, header sources, cookie sources, cross-service HTTP calls propagating taint to beego_admin.

### 4. beego_admin

**Purpose:** Admin panel demonstrating multi-hop taint flows (Controller -> Service -> Repository -> SQL).

**Framework:** beego v2

| Layer | SQLi | CmdI | PathTrav |
|-------|------|------|----------|
| Controller | 9 | 2 | 3 |
| Service | 8 | 1 | 1 |
| **Total** | **17** | **3** | **4** |

**Key patterns:** beego-specific sources (c.GetString, c.Ctx.Input.Param, c.Ctx.Input.Cookie, c.Ctx.Input.RequestBody), multi-hop 2-3 function chains, column name injection via UpdateUserVulnerable, arbitrary SQL execution via GenerateReport.

### 5. grpc_users

**Purpose:** gRPC service demonstrating taint from protobuf message fields.

**Framework:** gRPC, database/sql

| Layer | SQLi | CmdI | PathTrav |
|-------|------|------|----------|
| Server | 14 | 2 | 2 |
| Repository | 8 | 0 | 0 |
| **Total** | **22** | **2** | **2** |

**Key patterns:** gRPC request fields as taint sources (req.Id, req.Username, req.Query), metadata map values, ExecuteQuery allowing arbitrary SQL, column name injection, bulk delete in loop.

### 6. cobra_cli_test

**Purpose:** Cobra CLI tool. Minimal, no web surface.

**Framework:** cobra

**Vulnerabilities:** NONE. Clean CLI with proper flag validation (cobra.ExactArgs, typed getters). No database, no shell exec, no file I/O with user input.

---

## Audit Results (2026-03-22)

### Test File Quality

| Check | Result |
|-------|--------|
| Files exist | 476/476 |
| Function names match file numbers | 476/476 PASS |
| No vulnerability hints in comments | PASS (all clean -- testcode and apps) |
| CSV classifications match code | PASS (all 476 verified) |
| Package declarations correct | PASS |
| Imports valid | PASS |
| Shared helpers used | PASS (DB, RespondJSON, ParseJSONBody) |

### Known Issues

1. **cmd/main.go**: Does not register HTTP handlers via http.HandleFunc(). Tests are valid Go functions but the server returns 404 for all routes. Does NOT affect SAST scanning (TheAuditor indexes source, not runtime behavior).

2. **go.mod dependencies**: Lists dependencies for reference but go.sum not populated. Project needs `go mod tidy` to resolve. Again, does not affect SAST scanning.

---

## Scoring Formula

```
Score = TPR - FPR  (Youden's J statistic)

TP = ground_truth=true  AND tool flagged it
FN = ground_truth=true  AND tool missed it
FP = ground_truth=false AND tool flagged it
TN = ground_truth=false AND tool did NOT flag it

TPR = TP / (TP + FN)    (Sensitivity / Recall)
FPR = FP / (FP + TN)    (Fall-out)

Score range: -100% to +100%. 0% = random guessing.
```

---

## Scoring

Run any SAST tool, export SARIF 2.1.0, then score:

```bash
# Run your tool and export SARIF
your-tool scan ./testcode/ --output results.sarif

# Score against ground truth
python ../scripts/score_sarif.py results.sarif expectedresults-0.1.csv
```

The scorer computes both **category-averaged** (OWASP standard) and **flat aggregate** scores.

See [SCORING.md](SCORING.md) for full instructions, tool-specific SARIF export commands (Semgrep, Gosec, CodeQL, SonarQube), and scoring methodology details.

### TheAuditor Users

```bash
# Run TheAuditor
aud full --offline

# Convert proprietary DB to SARIF, then score
python ../scripts/convert_theauditor.py .pf/repo_index.db > theauditor.sarif
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.1.csv
```

---

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for full version history.

**Current: v0.3.1** -- 476 test cases, 21 CWE categories, 8 frameworks, 5 reference apps with 395 classified functions. Tool-agnostic SARIF scoring.
