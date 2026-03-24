# Go SAST Benchmark -- Development Roadmap

> **Status**: Prototype (~25% of OWASP-quality benchmark)
> **Last updated**: 2026-03-19
> **Owner**: Team Go

---

## 1. Honest Current State Assessment

### What Exists

| Asset | Count | Quality |
|-------|-------|---------|
| Test files (testcode/) | 256 | Mixed -- hand-written (1-110) decent, agent-written (111-256) variable |
| CSV answer key | 256 entries | Classifications verified correct |
| Reference apps (apps/) | 6 apps, 67 .go files | Fully audited, 100+ vulnerabilities documented |
| Documentation | go_benchmark.md (450 lines) | Test registry, app inventory, scoring script |
| Shared infra | shared.go, cmd/main.go, go.mod | Minimal -- main.go doesn't register handlers |

### What's Wrong

**Problem 1: No discrimination patterns.** OWASP Java's power comes from HARD false-positive tests. Their safe variants use dead-code ternaries, variable overwrites, map key confusion, and collection wrappers to look IDENTICAL to vulnerable code. Our safe variants are trivially obvious -- "use `?` instead of Sprintf" or "hardcode the path." Any tool that knows parameterized queries are safe gets a free pass. No tool is forced to prove it actually tracks data flow.

**Problem 2: Zero framework diversity in test cases.** All 256 tests use `net/http` standard library as their source API. The reference apps demonstrate patterns across gin, echo, chi, fiber, gorilla/mux, beego, and gRPC -- none of which appear in the test cases. A SAST tool that only supports net/http would score 100% on our benchmark while failing on every real Go project.

**Problem 3: No multi-hop cross-file flows.** Every test case is a single function in a single file. Real Go code has handler -> service -> repository chains (as shown in our own reference apps). OWASP Java has 1-hop, 2-hop, and 3-hop taint propagation chains. We have zero cross-file flows in the testcode.

**Problem 4: Scale.** OWASP Java has 2,740 test cases (504 for SQLi alone). We have 256 total (50 for SQLi). Per-category coverage is thin. Many vulnerability sub-patterns are unrepresented.

**Problem 5: No compilation verification.** We haven't verified that all 256 test files compile together as a Go package. Import conflicts, unused imports, and type errors may exist.

**Problem 6: Reference apps not integrated.** The 6 apps in `apps/` are copied material. They aren't scored against ground truth. They have existing TAINT SOURCE/SINK annotations which would need to be removed or converted to vuln-code-snippet format for benchmark use.

---

## 2. What OWASP Java Has That We Don't

### Discrimination Patterns (the entire point of OWASP benchmarks)

OWASP Java uses these patterns to make safe variants HARD to distinguish from vulnerable ones:

| Pattern | How it works | Our status |
|---------|-------------|------------|
| **Dead-code ternary** | `bar = (7*18)+num > 200 ? "safe_constant" : param` -- ternary always evaluates to constant branch, so `bar` is never tainted | MISSING |
| **Variable overwrite** | `bar = param; bar = "safe_value";` -- tainted value assigned then replaced before use at sink | Have 1 test (00024) -- need 20+ |
| **Map key confusion** | `map.put("keyA", param); bar = map.get("keyB");` -- tainted value stored under different key than retrieved | MISSING |
| **Collection wrapper** | `list.add(param); bar = list.get(0);` -- taint propagates through collection | MISSING |
| **Integer parse + use** | `intVal = Integer.parseInt(param); query = "...WHERE id=" + intVal` -- integer conversion makes string safe | Have 2 tests (00010, 00032) -- decent |
| **Properties/config** | `algorithm = properties.getProperty("hashAlg", "SHA-512")` -- value from config, may be weak depending on config | MISSING |
| **Multiple sources merge** | Two sources combine into one query -- both must be tainted for vuln | MISSING |
| **Safe-sink redirect** | Input reaches a safe API (parameterized query) that looks similar to a dangerous one | Have many -- this is our strongest area |

### Scale per Category

| Category | OWASP Java | Our Go Benchmark | Gap |
|----------|-----------|-----------------|-----|
| SQLi | 504 | 50 | 10x |
| CmdI | 251 | 30 | 8x |
| PathTrav | 268 | 30 | 9x |
| XSS | 455 | 20 | 22x |
| WeakRand | 493 | 20 | 24x |
| WeakHash | 236 | 20 | 12x |
| WeakCipher | 246 | 16 | 15x |
| SecureCookie | 67 | 16 | 4x |
| TrustBound | 126 | 0 | infinite |
| XPathI | 35 | 0 | infinite |
| LDAPi | 59 | 0 | infinite |

### Structural Consistency

OWASP Java: Every test case is a Java Servlet with `doPost(HttpServletRequest req, HttpServletResponse resp)`. Same source API (`req.getParameter()`), same template, same structure. Variations are in the DATA FLOW, not the framework.

Our benchmark: 256 test cases with varying signatures, varying source APIs, varying styles. No consistent template.

---

## 3. Reference App Audit Summary

### multi-api (2,709 lines, 5 frameworks)

| Category | Vulns | Safe | Unique Patterns |
|----------|-------|------|-----------------|
| SQLi | 40+ | 5 | fmt.Sprintf, string concat, Builder, Sprintf with LIKE, multi-hop through service, struct field propagation, fake sanitizer |
| CmdI | 12+ | 2 | exec.Command(user), sh -c, cross-function |
| PathTrav | 12+ | 0 | filepath.Join, direct os.Open, os.Create, URL path slice |
| XSS | 3 | 0 | template.HTML(), template.JS() |
| CORS | 2 | 0 | Header reflection |

**Key patterns for test cases**: Fake sanitizer (function returns input unchanged), multi-hop handler->service->repository, struct field taint distribution, multiple sinks from single source, goroutine closure taint, race conditions (async_service.go).

### calorie-tracker (30 files, gin + GORM)

| Category | Vulns | Safe |
|----------|-------|------|
| SQLi | 2 (db.Raw + Sprintf) | All other queries (GORM Where("?", val)) |

**Key pattern for test cases**: GORM `db.Raw(fmt.Sprintf(...))` vs GORM `db.Where("?", val)`. This is a critical discrimination test -- same ORM, two usage patterns, only one is vulnerable.

### go_notifications (16 files, gorilla/mux)

| Category | Vulns | Safe |
|----------|-------|------|
| SQLi | 14 | 2 |
| CmdI | 3 | 0 |
| PathTrav | 3 | 0 |
| SSRF | 1 (webhook URL) | 0 |
| Template Injection | 1 | 0 |
| Log Injection | 1 | 0 |
| Cross-Service Taint | 2 (HTTP calls to beego_admin) | 0 |

**Key patterns**: Cross-service taint propagation via HTTP, webhook URL SSRF, template name injection, log injection via newlines.

### beego_admin (5 files, beego v2)

| Category | Vulns | Safe |
|----------|-------|------|
| SQLi (controller) | 9 | 1 |
| SQLi (service) | 8 | 2 |
| CmdI | 3 | 0 |
| PathTrav | 4 | 0 |

**Key patterns**: Multi-hop controller->service->SQL, column name injection (`UPDATE %s SET %s`), arbitrary SQL execution (GenerateReport), weak validation bypass (length check doesn't sanitize SQL chars), beego-specific source APIs.

### grpc_users (3 files, gRPC)

| Category | Vulns | Safe |
|----------|-------|------|
| SQLi (server) | 14 | 0 |
| SQLi (repository) | 8 | 3 |
| CmdI | 2 | 0 |
| PathTrav | 2 | 0 |

**Key patterns**: gRPC protobuf message fields as taint sources, ExecuteQuery backdoor (raw SQL from client), metadata map iteration with tainted values, multi-hop server->repository.

### cobra_cli_test (1 file, cobra)

**CLEAN** -- no attack surface, no vulnerabilities, no database, no exec.

---

## 4. Workstream Breakdown

### WS-1: Discrimination Pattern Library (HIGHEST PRIORITY)

**Goal**: Add OWASP-style hard-FP patterns that force SAST tools to prove data flow tracking.

**Deliverables**:
- Dead-code ternary tests (at least 10 per flow category)
- Variable overwrite tests (at least 10)
- Map key confusion tests (at least 5)
- Collection/slice wrapper tests (at least 5)
- Constant propagation tests (at least 5)
- Fake sanitizer tests (at least 5)
- Multi-source merge tests (at least 5)

**Estimated new tests**: 45-60 safe variants using discrimination patterns

**Files**: New test files in testcode/ + CSV updates

### WS-2: Framework Source Diversity (HIGH PRIORITY)

**Goal**: Port existing test patterns to use gin, echo, chi, fiber, gorilla, beego, and gRPC source APIs.

**Approach**: For each framework, create 5-10 test cases that use framework-specific source APIs but flow to the same sinks. This tests whether the SAST tool's taint sources are framework-aware.

**Deliverables**:
- 5 tests using gin sources (c.Query, c.Param, c.ShouldBindJSON)
- 5 tests using echo sources (ctx.QueryParam, ctx.Param, ctx.Bind)
- 5 tests using chi sources (chi.URLParam)
- 5 tests using fiber sources (c.Params, c.BodyParser)
- 5 tests using gorilla sources (mux.Vars)
- 5 tests using beego sources (c.GetString, c.Ctx.Input)
- 3 tests using gRPC sources (protobuf message fields)

**Estimated new tests**: 33

**Files**: New test files + go.mod dependency updates

### WS-3: Cross-File Multi-Hop Flows (HIGH PRIORITY)

**Goal**: Add test cases that propagate taint across multiple files/functions, matching real Go application architecture.

**Deliverables**:
- 2-hop: handler -> service -> SQL (5 vulnerable, 5 safe)
- 3-hop: handler -> service -> helper -> SQL (3 vulnerable, 3 safe)
- Struct field propagation: handler binds JSON -> passes struct to service -> service uses field in query (3 vuln, 3 safe)
- Return value chain: service returns tainted value, handler uses it (3 vuln, 3 safe)

**Approach**: Create a `testcode/services/` subdirectory with helper functions that test cases call.

**Estimated new tests**: 28

**Files**: New test files + new service files

### WS-4: Test Case Quality Hardening (MEDIUM PRIORITY)

**Goal**: Fix existing 256 tests for consistency, compilation, and quality.

**Deliverables**:
- Verify all 256 files compile (`go vet ./testcode/...`)
- Fix import issues (unused imports, missing imports)
- Standardize function signatures
- Remove any accidental vulnerability hints in comments
- Ensure safe variants are non-trivially safe

**Files**: Edits to existing testcode/ files

### WS-5: Missing CWE Categories (MEDIUM PRIORITY)

**Goal**: Add categories that OWASP Java has but we don't.

| Category | CWE | Tests Needed | Go Pattern |
|----------|-----|-------------|------------|
| trustbound | 501 | 16 (8+8) | Context value from user stored in session |
| ldapi | 90 | 8 (4+4) | go-ldap library with string concat filter |
| nosql | 943 | 8 (4+4) | MongoDB bson.M with user-controlled keys |
| loginjection | 117 | 8 (4+4) | log.Printf with user newlines |

**Estimated new tests**: 40

### WS-6: Scale Expansion (LOW PRIORITY -- after WS-1 through WS-5)

**Goal**: Expand test count per category toward OWASP Java ratios.

Focus on SQLi (from 50 to 150) and CmdI/PathTrav (from 30 to 80 each). Each new test should cover a pattern not already tested.

### WS-7: Reference App Integration (LOW PRIORITY -- future version)

**Goal**: Make reference apps scoreable with vuln-code-snippet annotations and app-specific ground truth files.

This is the most complex workstream -- each app needs its own expectedresults.csv with every vulnerability classified. Deferred to v0.3.1+.

---

## 5. Missing Test Patterns Per Category

### SQL Injection (CWE-89) -- 50 existing, need 100+

**Have**: Sprintf, concat, Builder, Join, cross-function, struct field, map iteration, conditional, loop, interface cast, header/cookie source, fake sanitizer, second-order, goroutine, dead variable, UUID validation, allowlist, integer parse, hash-before-use.

**Missing**:
- GORM db.Raw() vs db.Where() discrimination
- sqlx named query vs string format
- pgx parameterized vs concat
- Dead-code ternary (always-constant branch)
- Variable overwrite (assign tainted, then overwrite with safe)
- Map key confusion (put under key A, get from key B)
- Slice/array wrapper (append tainted, index safe)
- Channel send/receive taint propagation
- Multi-hop through 3+ function calls
- Cross-file handler -> service -> repository
- Struct method receiver taint
- Interface implementation dispatch
- Error path taint (err.Error() in query)
- Deferred function taint
- Closure capture taint (non-goroutine)
- Multipart form file metadata (header.Filename in query)
- URL fragment/scheme injection
- Batch INSERT with VALUES builder
- RETURNING clause injection
- CTE (WITH clause) injection

### Command Injection (CWE-78) -- 30 existing, need 60+

**Missing**:
- syscall.Exec (direct syscall, not os/exec)
- os.StartProcess
- Dead-code branch where exec never runs
- Variable overwrite before exec
- Allowlisted args with shell (still vulnerable)
- TOCTOU: check file, then exec with filename
- Plugin/dynamic loading with user path
- FFI/cgo with user input

### Path Traversal (CWE-22) -- 30 existing, need 60+

**Missing**:
- Zip slip (archive extraction with user entry names)
- Symlink following
- Double URL encoding (%252e%252e)
- Null byte injection (%00)
- Backslash on Windows (`..\..\`)
- filepath.EvalSymlinks as defense
- chroot/jail pattern
- io/fs.FS interface (embedded vs OS)
- http.Dir restrictions

---

## 6. Milestone Roadmap

### v0.2 -- Quality Foundation (WS-1 + WS-4)
- Add 45-60 discrimination pattern tests
- Fix existing 256 for compilation and consistency
- Target: 300+ tests, all compile, OWASP-style FP discrimination
- **This is the minimum for not getting laughed at**

### v0.3 -- Framework Diversity (WS-2 + WS-3)
- Add 33 framework-specific tests
- Add 28 cross-file multi-hop tests
- Target: 360+ tests, 7 frameworks tested, cross-file flows
- **This is the minimum for being useful to Go SAST tool authors**

### v0.4 -- Category Expansion (WS-5)
- Add 40 tests for missing CWE categories
- Target: 400+ tests, 16 CWE categories
- **This matches OWASP Python benchmark scope**

### v0.5 -- Scale (WS-6)
- Expand to 500+ tests with deeper per-category coverage
- Focus SQLi (150), CmdI (80), PathTrav (80)
- **This is credible for public release**

### v1.0 -- OWASP Quality (WS-7 + community iteration)
- Reference app integration with ground truth
- 1000+ tests
- Community contributions
- CI scoring pipeline
- **This is the goal**

---

## 7. How to Use This Roadmap

Each workstream becomes a separate plan-mode session:

1. Enter plan mode
2. Reference this roadmap for the specific WS scope
3. Read relevant test files and reference apps
4. Design test cases with full discrimination patterns
5. Execute with proper documentation
6. Update go_benchmark.md with new test registry entries
7. Update expectedresults CSV
8. Verify compilation
9. Update this roadmap with completion status

**One workstream at a time. No cowboy coding. Every test documented.**

---

## 8. Workstream Status Tracker

| WS | Name | Status | Tests Added | Tests Fixed |
|----|------|--------|-------------|-------------|
| 1 | Discrimination Patterns | COMPLETE | 36 | 0 |
| 2 | Framework Diversity | COMPLETE | 33 | 0 |
| 3 | Cross-File Flows | COMPLETE | 17 (+services.go) | 0 |
| 4 | Quality Hardening | COMPLETE | 0 | 1 (00074 init hack) |
| 5 | Missing CWEs | COMPLETE | 32 (4 new categories) | 0 |
| 6 | Scale Expansion | COMPLETE | 50 (GORM,sqlx,zip,syscall,WS,SSE) | 0 |
| 7 | App Integration | COMPLETE | 395 functions across 5 apps | 0 |
