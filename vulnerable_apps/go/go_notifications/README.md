# Go Notifications Service

A multi-channel notification service written in Go. Part of Project Anarchy test suite for TheAuditor.

## Features

- Multiple notification channels: Email, Webhook, Slack, File
- Template rendering with Go templates
- Background job queue
- SQLite persistence
- Shell hook integration

## Architecture

```
go_notifications/
├── cmd/server/main.go       # Entry point
├── internal/
│   ├── api/                 # HTTP handlers (taint sources)
│   │   ├── handlers.go      # Request handlers
│   │   ├── router.go        # Route configuration
│   │   └── middleware.go    # Auth, logging, CORS
│   ├── channels/            # Notification sinks
│   │   ├── email.go         # SMTP delivery
│   │   ├── webhook.go       # HTTP webhook (SSRF)
│   │   ├── slack.go         # Slack integration
│   │   └── file.go          # File logging (path traversal)
│   ├── templates/           # Template rendering (SSTI)
│   ├── queue/               # Background workers
│   ├── storage/             # SQLite (SQL injection)
│   └── config/              # Configuration loading
├── pkg/utils/               # Utility functions
├── scripts/hooks/           # Shell hooks (command injection)
├── templates/               # Email templates
└── config/                  # Config files
```

## Running the Service

```bash
# Install dependencies
go mod download

# Run the server
go run ./cmd/server/main.go

# Or build and run
go build -o notify ./cmd/server
./notify
```

Server runs on `:8082` by default.

## API Endpoints

### Public

- `GET /api/health` - Health check
- `GET /api/debug` - Debug request info
- `POST /api/callback/{id}` - Webhook callback receiver

### Protected (requires X-API-Key header)

- `POST /api/notify` - Send immediate notification
- `POST /api/notify/batch` - Send batch notifications
- `POST /api/notify/template` - Send templated notification
- `GET /api/notifications` - List notifications (supports filtering)
- `GET /api/notifications/search` - Full-text search
- `GET /api/notifications/export` - Export notifications
- `POST /api/webhook/test` - Test webhook delivery
- `POST /api/hooks/execute` - Execute shell hook
- `GET /api/logs/{filename}` - Read log file
- `POST /api/config/import` - Import configuration
- `GET /api/proxy` - Proxy requests

## Example Requests

### Send Notification

```bash
curl -X POST http://localhost:8082/api/notify \
  -H "X-API-Key: dev-api-key-12345" \
  -H "Content-Type: application/json" \
  -d '{
    "channel": "webhook",
    "recipient": "http://example.com/webhook",
    "subject": "Test Alert",
    "message": "This is a test notification"
  }'
```

### Send Templated Notification

```bash
curl -X POST http://localhost:8082/api/notify/template \
  -H "X-API-Key: dev-api-key-12345" \
  -H "Content-Type: application/json" \
  -d '{
    "template": "notification.html",
    "channel": "email",
    "recipient": "user@example.com",
    "subject": "Hello",
    "data": {
      "name": "John",
      "message": "Welcome!"
    }
  }'
```

### Test Webhook (SSRF)

```bash
curl -X POST http://localhost:8082/api/webhook/test \
  -H "X-API-Key: dev-api-key-12345" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "http://internal-service:8080/admin",
    "method": "GET",
    "headers": {"X-Custom": "value"}
  }'
```

## Intentional Vulnerabilities (For Testing)

This service contains intentional security vulnerabilities for testing code analysis tools:

| Vulnerability | Location | Description |
|---------------|----------|-------------|
| SQL Injection | storage/db.go | User input in ORDER BY, LIKE clauses |
| Command Injection | handlers.go, hooks/*.sh | Shell commands with user input |
| Path Traversal | handlers.go, file.go | Arbitrary file read/write |
| SSRF | webhook.go, handlers.go | Unvalidated URL requests |
| Template Injection | templates/renderer.go | SSTI with dangerous functions |
| XSS | Templates, text/template | Unescaped user output |
| Hardcoded Secrets | config.go, config.yaml | API keys, passwords in code |
| Weak Crypto | utils/helpers.go | MD5 for passwords |
| Log Injection | middleware.go | User input in logs |
| Timing Attack | middleware.go, db.go | Non-constant-time comparison |
| CORS Misconfiguration | middleware.go | Reflects any origin |
| Missing Rate Limiting | config | Disabled by default |

## Data Flow (Taint Analysis)

```
HTTP Request (Source)
       │
       ▼
┌──────────────────┐
│   handlers.go    │  Parse JSON body, query params, headers
└────────┬─────────┘
         │
    ┌────┴────┬─────────────┬──────────────┐
    │         │             │              │
    ▼         ▼             ▼              ▼
┌───────┐ ┌───────┐   ┌──────────┐   ┌─────────┐
│ email │ │webhook│   │ storage  │   │templates│
│  .go  │ │  .go  │   │  db.go   │   │renderer │
└───────┘ └───────┘   └──────────┘   └─────────┘
    │         │             │              │
    ▼         ▼             ▼              ▼
  SMTP      HTTP         SQLite        File I/O
 (sink)   (SSRF sink)  (SQLi sink)   (path sink)
```

## Integration with Gateway

Add to gateway routes to enable cross-service taint flows:

```javascript
// In gateway/src/index.js
app.use('/api/notifications', createProxyMiddleware({
  target: 'http://localhost:8082',
  pathRewrite: { '^/api/notifications': '/api' }
}));
```

## Environment Variables

- `NOTIFY_LISTEN_ADDR` - Override listen address
- `NOTIFY_API_KEY` - Override API key
- `NOTIFY_DATABASE_PATH` - Override database path

## Testing Cross-Boundary Flows

```bash
# Via Gateway (frontend → gateway → go_notifications)
curl "http://localhost:4000/api/notifications/notify" \
  -H "Content-Type: application/json" \
  -d '{"channel":"webhook","recipient":"http://169.254.169.254/latest/meta-data/"}'

# Direct SQL Injection
curl "http://localhost:8082/api/notifications?order_by=id;DROP%20TABLE%20users;--"

# Template Injection
curl -X POST http://localhost:8082/api/notify/template \
  -H "X-API-Key: dev-api-key-12345" \
  -d '{"template":"{{shell \"id\"}}","channel":"file","recipient":"output.txt"}'
```
