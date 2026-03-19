# Vulnerable Go API - TheAuditor Test Fixture

This project is a comprehensive Go test fixture for TheAuditor's taint analysis, rule detection, and graph strategy testing.

## Purpose

This fixture is designed to exercise all Go-related features of TheAuditor:

1. **AST Extraction** (`go_impl.py`)
   - Package/import extraction
   - Struct and interface definitions
   - Function and method declarations
   - Variable and constant extraction
   - Goroutines, channels, and defer statements
   - Type parameters (generics)
   - Error return patterns

2. **Taint Analysis** (`discovery.py`, `ifds_analyzer.py`)
   - HTTP request sources (query params, path params, body, headers, cookies)
   - SQL injection sinks (Query, Exec, Raw, etc.)
   - Command injection sinks (exec.Command, os.StartProcess)
   - Template injection sinks (template.HTML, template.JS)
   - Path traversal sinks (os.Open, os.ReadFile, filepath.Join)

3. **Graph Strategies** (`go_http.py`, `go_orm.py`)
   - HTTP route registration patterns
   - Middleware chain detection
   - GORM relationship extraction
   - Entry/exit point edge emission

4. **Rule Detection** (`injection_analyze.py`, `concurrency_analyze.py`, etc.)
   - SQL injection via fmt.Sprintf and string concatenation
   - Command injection via exec.Command
   - Template injection via unsafe type conversions
   - Path traversal via user-controlled paths
   - Race conditions via captured loop variables

## Project Structure

```
vulnerable-api/
├── go.mod                          # Module definition
├── cmd/
│   └── server/
│       └── main.go                 # Main entry point
├── internal/
│   ├── handlers/
│   │   ├── gin_handlers.go         # Gin framework handlers
│   │   ├── echo_handlers.go        # Echo framework handlers
│   │   ├── chi_handlers.go         # Chi router handlers
│   │   ├── fiber_handlers.go       # Fiber framework handlers
│   │   ├── nethttp_handlers.go     # Standard net/http handlers
│   │   ├── routes.go               # Route registration patterns
│   │   └── middleware.go           # Middleware definitions
│   ├── models/
│   │   └── user.go                 # GORM models with relationships
│   ├── repository/
│   │   └── user_repository.go      # SQL injection vulnerabilities
│   └── services/
│       └── async_service.go        # Goroutines, channels, concurrency
```

## Covered Frameworks

| Framework | Taint Sources | Route Patterns |
|-----------|---------------|----------------|
| Gin | c.Query, c.Param, c.PostForm, c.ShouldBind | r.GET, r.POST, r.Group |
| Echo | c.QueryParam, c.Param, c.FormValue, c.Bind | e.GET, e.POST, e.Group |
| Chi | chi.URLParam, r.URL.Query, r.FormValue | r.Get, r.Post, r.Route |
| Fiber | c.Query, c.Params, c.Body, c.BodyParser | app.Get, app.Post, app.Group |
| net/http | r.URL.Query, r.FormValue, r.Body | http.HandleFunc |

## Vulnerability Categories

### SQL Injection (CWE-89)
- `fmt.Sprintf` with SQL keywords
- String concatenation building queries
- User input in table/column names
- ORM raw query methods

### Command Injection (CWE-78)
- `exec.Command` with variable command
- Shell command via `sh -c`
- User input in command arguments

### Template Injection / XSS (CWE-79)
- `template.HTML()` with user input
- `template.JS()` with user input
- Bypassing auto-escaping

### Path Traversal (CWE-22)
- `filepath.Join` with `../`
- Direct file path from user input
- `os.Open`, `os.ReadFile`, `os.Create`

### Race Conditions (CWE-362)
- Captured loop variables in goroutines
- Unsynchronized map access
- Missing mutex on shared state

## Database Tables Exercised

| Table | Source |
|-------|--------|
| go_packages | Package declarations |
| go_imports | Import statements |
| go_structs | Type struct definitions |
| go_struct_fields | Struct field tags (GORM) |
| go_interfaces | Interface definitions |
| go_interface_methods | Interface method signatures |
| go_functions | Function declarations |
| go_methods | Method declarations (with receivers) |
| go_func_params | Function parameters |
| go_func_returns | Return types |
| go_variables | Variable declarations |
| go_constants | Constant declarations |
| go_goroutines | go statements |
| go_channels | make(chan) calls |
| go_channel_ops | Send/receive operations |
| go_defer_statements | defer statements |
| go_captured_vars | Variables captured by closures |
| go_error_returns | Functions returning error |
| go_routes | HTTP route registrations |
| go_middleware | Middleware chain |

## Usage with TheAuditor

```bash
# Index the project
cd tests/go/vulnerable-api
aud full --index

# Run taint analysis
aud taint

# Check for injection vulnerabilities
aud rules go_injection

# Analyze concurrency issues
aud rules go_concurrency

# Query the database
aud query --file internal/handlers/gin_handlers.go --list all
aud query --symbol GetUser --show-callers
```

## Expected Findings

When analyzing this fixture, TheAuditor should detect:

1. **25+ SQL injection vulnerabilities** across all handlers
2. **10+ command injection vulnerabilities** via exec.Command
3. **5+ template injection vulnerabilities** via template.HTML/JS
4. **8+ path traversal vulnerabilities** via file operations
5. **5+ race conditions** from captured loop variables
6. **Full GORM relationship graph** from model definitions
7. **Complete route graph** with entry points per framework
