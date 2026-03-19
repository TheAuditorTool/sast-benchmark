# Go SAST Benchmark v0.3

## Overview

OWASP-style SAST benchmark for Go. **424 test cases** across **16 CWE categories** with 50/50 vulnerable/safe split (212/212). Plus 5 reference apps with 395 classified functions.

**Design principle**: Test cases written from security knowledge, NOT from knowledge of any specific SAST engine's detection capabilities. No vulnerability hints in source code. The CSV answer key is the ONLY ground truth.

**Audit status**: All 424 test files verified 2026-03-19. Zero vulnerability hints. Zero duplicate types/functions.

---

## Structure

```
gorustbash_benchmark/go/
  expectedresults-0.1.csv     # Answer key: test,category,vulnerable,CWE
  go_benchmark.md             # This file
  CHANGELOG.md                # Every change documented
  testcode/                   # 424 benchmark test files + shared.go + benchmark_services.go
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
| 11 | tlsverify | 295 | 5 | 5 | 10 |
| 12 | loginjection | 117 | 4 | 4 | 8 |
| 13 | nosql | 943 | 4 | 4 | 8 |
| 14 | ldapi | 90 | 4 | 4 | 8 |
| 15 | trustbound | 501 | 4 | 4 | 8 |
| 16 | deserial | 502 | 4 | 4 | 8 |
| | **TOTAL** | | **212** | **212** | **424** |

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

## Audit Results (2026-03-19)

### Test File Quality

| Check | Result |
|-------|--------|
| Files exist | 424/424 |
| Function names match file numbers | 424/424 PASS |
| No vulnerability hints in comments | PASS (all clean) |
| CSV classifications match code | PASS (all 424 verified via 3-agent full audit) |
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

## Scoring Script

After running `aud full --offline` on this benchmark project, score with:

```bash
/mnt/c/Users/santa/Desktop/TheAuditorV2/.venv/Scripts/python.exe -c '
import sqlite3
from collections import defaultdict

expected = {}
with open(r"C:\Users\santa\Desktop\open_tests\gorustbash_benchmark\go\expectedresults-0.1.csv") as f:
    for line in f:
        line = line.strip()
        if line.startswith("#") or not line: continue
        p = line.split(",")
        expected[p[0]] = {"cat": p[1], "real": p[2].lower() == "true", "cwe": int(p[3])}

NOISE = {"deadcode-function", "api-missing-auth"}
FILE_PREFIX = "testcode/benchmark_test_"

conn = sqlite3.connect(r"C:\Users\santa\Desktop\open_tests\gorustbash_benchmark\go\.pf\repo_index.db")
c = conn.cursor()
det = defaultdict(set)

c.execute("SELECT file, rule FROM pattern_findings WHERE file LIKE ? || ?", (FILE_PREFIX, "%"))
for f, r in c.fetchall():
    if r not in NOISE:
        name = f.split("/")[-1].replace(".go", "")
        name = name.replace("benchmark_test_", "BenchmarkTest")
        det[name].add(r)

c.execute("SELECT sink_file, vulnerability_type FROM resolved_flow_audit WHERE status = ? AND sink_file LIKE ? || ?", ("VULNERABLE", FILE_PREFIX, "%"))
for f, vt in c.fetchall():
    name = f.split("/")[-1].replace(".go", "")
    name = name.replace("benchmark_test_", "BenchmarkTest")
    det[name].add("taint:" + vt)

conn.close()

cats = sorted(set(e["cat"] for e in expected.values()))
print("%-16s %-6s %-5s %-5s %-5s %-5s %7s %7s %7s" % ("Category","CWE","TP","FP","FN","TN","TPR","FPR","Score"))
print("-" * 75)
ttp=tfp=tfn=ttn=0
for cat in cats:
    tests = {k:v for k,v in expected.items() if v["cat"]==cat}
    tp=fp=fn=tn=0
    for t,i in tests.items():
        d = len(det.get(t, set())) > 0
        if i["real"] and d: tp+=1
        elif i["real"]: fn+=1
        elif d: fp+=1
        else: tn+=1
    tr=tp+fn; ts=fp+tn
    tpr=tp/tr if tr else 0; fpr=fp/ts if ts else 0
    ttp+=tp;tfp+=fp;tfn+=fn;ttn+=tn
    print("%-16s %-6d %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%" % (
        cat,list(tests.values())[0]["cwe"],tp,fp,fn,tn,tpr*100,fpr*100,(tpr-fpr)*100))
otpr=ttp/(ttp+tfn) if (ttp+tfn) else 0; ofpr=tfp/(tfp+ttn) if (tfp+ttn) else 0
print("-" * 75)
print("%-16s %-6s %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%" % (
    "OVERALL","",ttp,tfp,tfn,ttn,otpr*100,ofpr*100,(otpr-ofpr)*100))
'
```

---

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for full version history.

**Current: v0.3** -- 424 test cases, 16 CWE categories, 7 frameworks, 5 reference apps with 395 classified functions.
