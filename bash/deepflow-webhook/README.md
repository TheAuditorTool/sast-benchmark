# deepflow-webhook

A pure bash HTTP webhook server for deployment automation. This project demonstrates various taint flows and security patterns in bash scripts for static analysis testing.

## Structure

```
deepflow-webhook/
  server.sh             # Main HTTP server (nc/socat-based)
  config.sh             # Configuration variables
  handlers/
    webhook.sh          # Main request router and processor
    deploy.sh           # Deployment actions
    notify.sh           # Notification dispatcher
  lib/
    http.sh             # HTTP request/response parsing
    json.sh             # JSON parsing utilities
    validate.sh         # Input validation functions
```

## Usage

```bash
# Start server with defaults (port 8080)
./server.sh

# Start on custom port
./server.sh -p 9000

# Start in safe mode (disables dangerous endpoints)
./server.sh --safe

# Test single request
echo -e "POST /webhook HTTP/1.1\r\nContent-Length: 2\r\n\r\n{}" | ./server.sh --single
```

## Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/webhook` | POST | GitHub webhook receiver |
| `/deploy` | POST | Trigger deployments |
| `/notify` | POST | Send notifications |
| `/exec` | POST | Execute commands (DANGEROUS) |
| `/query` | POST | Execute SQL queries (DANGEROUS) |
| `/status` | GET | Server status |
| `/health` | GET | Health check |

## Taint Flows (For Security Analysis)

### Command Injection

| Source | Sink | Location |
|--------|------|----------|
| `QUERY_STRING` | `eval` | handlers/webhook.sh |
| JSON `.command` | `eval` | handlers/webhook.sh:handle_exec |
| JSON `.callback` | `eval` | handlers/notify.sh:notify_custom |
| JSON `.script` | `bash -c` | handlers/deploy.sh:do_deploy |
| `.deployment.task` | `eval` | handlers/webhook.sh:handle_deployment_event |

### SQL Injection

| Source | Sink | Location |
|--------|------|----------|
| JSON `.query` | `mysql -e` | handlers/webhook.sh:handle_query |

### Path Traversal

| Source | Sink | Location |
|--------|------|----------|
| JSON `.target` | `rm -rf` | handlers/deploy.sh:do_deploy |
| JSON `.target` | `cp -r` | handlers/deploy.sh:do_rollback |

### SSRF / Data Exfiltration

| Source | Sink | Location |
|--------|------|----------|
| JSON `.webhook_url` | `curl` | handlers/notify.sh:notify_slack |
| JSON URL | `curl` | handlers/notify.sh:notify_webhook |
| JSON `.endpoint` | `curl` | handlers/notify.sh:notify_with_data |
| JSON `.url` | `curl` | handlers/deploy.sh:deploy_from_url |

## Sanitizers

The `lib/validate.sh` module provides sanitization functions:

| Function | Sanitizer Type | Pattern |
|----------|---------------|---------|
| `validate_action` | Whitelist | `case` statement |
| `validate_path` | Canonicalization | `realpath` |
| `sanitize_shell` | Shell escaping | `printf '%q'` |
| `validate_regex` | Pattern matching | `[[ =~ ]]` |
| `validate_branch` | Input validation | Regex + disallow `..` |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8080 | Listen port |
| `HOST` | 0.0.0.0 | Listen address |
| `SAFE_MODE` | false | Disable dangerous endpoints |
| `WEBHOOK_SECRET` | (empty) | GitHub webhook secret |
| `DEPLOY_DIR` | /var/deployments | Deployment directory |
| `SLACK_WEBHOOK_URL` | (empty) | Slack notification URL |
| `DB_HOST` | localhost | Database host |
| `DB_USER` | webhook | Database user |
| `DB_NAME` | deployments | Database name |

## Testing

```bash
# Health check
curl http://localhost:8080/health

# Status
curl http://localhost:8080/status

# Trigger deployment
curl -X POST http://localhost:8080/deploy \
  -H "Content-Type: application/json" \
  -d '{"action": "deploy", "target": "myapp", "script": "npm install"}'

# Send notification
curl -X POST http://localhost:8080/notify \
  -H "Content-Type: application/json" \
  -d '{"channel": "slack", "message": "Deployment complete"}'
```

## Security Notes

This project intentionally contains vulnerable code patterns for static analysis testing. DO NOT use in production.

Vulnerable patterns included:
- Direct `eval` of user input
- Unvalidated file paths
- SQL query construction from user input
- SSRF via user-controlled URLs
