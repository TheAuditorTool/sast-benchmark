# Rust JobQueue

A comprehensive distributed job queue system written in Rust. This project serves as a **real-world test fixture** for code auditing tools, containing intentional vulnerabilities and covering extensive Rust patterns.

## Architecture

```
rust_jobqueue/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── jobqueue-core/            # Core types, traits, errors
│   │   └── src/
│   │       ├── lib.rs            # Module exports
│   │       ├── job.rs            # Job struct, builder pattern
│   │       ├── error.rs          # Error types, Result alias
│   │       ├── traits.rs         # JobExecutor, JobStore traits
│   │       ├── priority.rs       # Priority enum, PriorityQueue
│   │       ├── types.rs          # WorkerId, QueueName, newtypes
│   │       ├── config.rs         # Configuration structures
│   │       ├── events.rs         # Event system, EventBus
│   │       ├── metrics.rs        # Metrics collection
│   │       └── result.rs         # JobResult, BatchResult
│   │
│   ├── jobqueue-db/              # Database layer (SQLite)
│   │   └── src/
│   │       ├── lib.rs            # DbError, DbResult
│   │       ├── sqlite.rs         # SqliteStore implementation
│   │       ├── migrations.rs     # Schema migrations
│   │       ├── pool.rs           # Connection pooling
│   │       ├── queries.rs        # Query builders (VULNERABLE)
│   │       └── backup.rs         # Backup/restore (VULNERABLE)
│   │
│   ├── jobqueue-worker/          # Worker implementation
│   │   └── src/
│   │       ├── lib.rs            # Module exports
│   │       ├── worker.rs         # Main Worker struct
│   │       ├── executor.rs       # ExecutorPool, ThreadPool
│   │       ├── scheduler.rs      # Job scheduling, cron
│   │       ├── health.rs         # Health checks
│   │       ├── handlers.rs       # Built-in handlers (VULNERABLE)
│   │       └── middleware.rs     # Job middleware
│   │
│   ├── jobqueue-api/             # HTTP API (Axum)
│   │   └── src/
│   │       ├── lib.rs            # Module exports
│   │       ├── server.rs         # ApiServer, configuration
│   │       ├── routes.rs         # Route handlers (VULNERABLE)
│   │       ├── handlers.rs       # Response helpers
│   │       ├── middleware.rs     # Auth, logging, rate limiting
│   │       ├── auth.rs           # Authentication (VULNERABLE)
│   │       └── error.rs          # API errors
│   │
│   └── jobqueue-cli/             # CLI binary
│       └── src/
│           ├── main.rs           # Clap CLI definition
│           └── commands/
│               ├── mod.rs
│               ├── server.rs     # Start API server
│               ├── worker.rs     # Start worker
│               ├── job.rs        # Job management
│               ├── queue.rs      # Queue management
│               ├── db.rs         # Database operations
│               ├── status.rs     # System status
│               └── shell.rs      # Interactive shell
```

## Quick Start

```bash
cd frameworks/rust_jobqueue

# Build everything
cargo build

# Run the CLI help
cargo run --bin jq -- --help

# Start the API server
cargo run --bin jq -- server --port 8080

# In another terminal, start a worker
cargo run --bin jq -- worker --queues default --concurrency 4

# Create a job
cargo run --bin jq -- job create --type echo --data '{"message": "hello"}'

# List jobs
cargo run --bin jq -- job list

# Check status
cargo run --bin jq -- status
```

## API Endpoints

```
POST   /jobs              - Create a job
GET    /jobs              - List jobs
GET    /jobs/:id          - Get job details
DELETE /jobs/:id          - Cancel a job
POST   /jobs/:id/retry    - Retry a job

GET    /queues            - List queues
GET    /queues/:name/stats - Queue statistics
POST   /queues/:name/pause - Pause queue
POST   /queues/:name/resume - Resume queue

GET    /admin/search      - Search jobs (SQL injection)
POST   /admin/sql         - Execute raw SQL (DANGEROUS)
POST   /admin/cleanup     - Clean old jobs

GET    /health            - Health check
GET    /health/live       - Liveness probe
GET    /health/ready      - Readiness probe
GET    /metrics           - Prometheus metrics
```

## Intentional Vulnerabilities

This project contains intentional security vulnerabilities for SAST testing:

### SQL Injection
- `sqlite.rs:search_jobs()` - Search term interpolated directly
- `sqlite.rs:get_jobs_by_tag()` - Tag name in SQL
- `sqlite.rs:list_jobs_ordered()` - ORDER BY column from user
- `queries.rs:QueryBuilder` - All methods vulnerable
- `queries.rs:bulk_insert_sql()` - Values not escaped

### Command Injection
- `handlers.rs:ShellHandler` - Executes user-controlled commands
- `backup.rs:export_to_sql()` - Shell command with user path
- `backup.rs:import_from_sql()` - Shell command with user path
- `backup.rs:compress_backup()` - Backup name in command

### Path Traversal
- `backup.rs:create_backup()` - User-controlled backup name
- `backup.rs:restore_backup()` - User-controlled backup path
- `backup.rs:read_backup_contents()` - Arbitrary file read
- `backup.rs:write_backup()` - Arbitrary file write
- `handlers.rs:FileHandler` - Path not sanitized

### SSRF (Server-Side Request Forgery)
- `handlers.rs:HttpHandler` - Fetches user-controlled URLs
- `handlers.rs:WebhookHandler` - Posts to user-controlled URLs

### Authentication Issues
- `auth.rs:hash_password()` - Uses MD5
- `auth.rs:authenticate()` - Timing attack (early return)
- `auth.rs:generate_token()` - Predictable tokens
- `auth.rs:JwtToken` - Uses "none" algorithm
- `middleware.rs:AuthMiddleware` - Logs invalid API keys

### Memory Safety
- `pool.rs:PoolGuard::drop()` - Blocking lock in async context
- `middleware.rs:ThreadIdExt` - Unsafe transmute
- `executor.rs:ThreadPool` - No panic catching

### Other Issues
- `events.rs:WebhookHandler` - MD5 for HMAC
- `config.rs:BasicAuthConfig` - Plaintext password
- `config.rs:EnvConfig::api_key()` - Logs secrets
- `routes.rs:execute_sql()` - Reflects user input (XSS)
- `error.rs` - Leaks internal details
- `server.rs` - Permissive CORS (Any origin)
- `scheduler.rs:CronExpression` - Panics on invalid input

## Rust Patterns Covered

This codebase covers extensive Rust patterns for indexer testing:

### Types & Traits
- Structs with derive macros
- Enums with variants
- Trait definitions and implementations
- Generic types and bounds
- Associated types
- Trait objects (`Box<dyn Trait>`)
- Phantom data

### Ownership & Lifetimes
- References and borrowing
- Lifetime annotations
- Arc/Rc for shared ownership
- Mutex/RwLock for interior mutability

### Async/Await
- Async functions and traits
- Tokio runtime
- Channels (mpsc, broadcast, oneshot)
- Futures and polling
- Timeouts and intervals

### Modules & Visibility
- Module hierarchy
- Re-exports (`pub use`)
- Workspace dependencies
- Feature flags

### Macros
- Derive macros (Serialize, Deserialize, etc.)
- Function-like macros
- Custom error macro

### Error Handling
- Result and Option
- Custom error types with thiserror
- Error conversion (From trait)
- Error context

### Patterns
- Builder pattern
- Newtype pattern
- Extension traits
- Middleware pattern

## File Count & Lines

```
Crate             Files    Lines (approx)
────────────────  ─────    ──────────────
jobqueue-core       9         ~2000
jobqueue-db         5         ~1200
jobqueue-worker     6         ~1400
jobqueue-api        6         ~1100
jobqueue-cli        8         ~800
────────────────  ─────    ──────────────
Total              34         ~6500
```

## Test Coverage

Each crate includes unit tests. Run them with:

```bash
cargo test --workspace
```

## Dependencies

Major dependencies:
- `tokio` - Async runtime
- `axum` - Web framework
- `rusqlite` - SQLite database
- `serde` - Serialization
- `clap` - CLI framework
- `tracing` - Logging
- `chrono` - Date/time
- `uuid` - UUIDs

## Purpose

This project exists to test:

1. **Rust Parser Accuracy** - Can your tool parse all Rust syntax correctly?
2. **Import Resolution** - Does it resolve workspace dependencies?
3. **Trait Analysis** - Can it track trait implementations?
4. **Async Analysis** - Does it understand async/await patterns?
5. **Taint Analysis** - Can it trace data from source to sink?
6. **Vulnerability Detection** - Does it find the intentional bugs?
