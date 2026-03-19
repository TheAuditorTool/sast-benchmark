# Go Benchmark Changelog

## v0.3.2 (2026-03-20)

### Personal Read Verification
- Manually read 24 additional test files focusing on edge cases and debatable classifications
- Fixed BenchmarkTest00146 (ssrf=false): IP validation was applied to `host` param but `http.Get` fetched separate `targetURL` param. Validation now applies to the URL actually being fetched via `url.Parse(targetURL).Hostname()`.
- Fixed BenchmarkTest00398 (cmdi=true): Used `exec.Command("cat", file)` with hardcoded command -- same pattern as tests 054/066 which are correctly classified as cmdi=false. Changed to `exec.Command("sh", "-c", "cat "+file)` so shell metacharacters in `file` CAN execute injected commands, making the cmdi=true classification correct.

## v0.3.1 (2026-03-19)

### Deep Verification Pass
- 3 audit agents verified all 314 agent-written test files against CSV classifications
- 99.4% accuracy (312/314 correct)
- Fixed BenchmarkTest00118 (xss): reclassified true->false. Go's encoding/json escapes HTML by default (`<` -> `\u003c`). Even with Content-Type text/html, json.Encoder output cannot inject HTML tags.
- Fixed BenchmarkTest00255 (deserial): removed user-controlled sessionID from file path. The gob.Decode to typed struct was safe, but os.Open path was built from query param (path traversal). Changed to hardcoded internal path.
- TP/TN split improved from 213/211 to 212/212 (perfect 50/50)
- Updated all documentation to reflect corrected numbers

## v0.3 (2026-03-19)

### Scale Expansion (+50 tests)
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
- Ground truth CSVs for 5 reference apps (395 classified functions total)
- vulnerable-api: 115 entries
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
