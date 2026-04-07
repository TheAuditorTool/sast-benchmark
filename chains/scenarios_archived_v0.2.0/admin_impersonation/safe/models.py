"""Data models for the internal support tooling service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "support-tool-secret"

# user_id -> {username, role, email}
USERS = {
    "usr-001": {"username": "alice", "role": "user", "email": "alice@example.com"},
    "usr-002": {"username": "bob", "role": "user", "email": "bob@example.com"},
    "adm-001": {"username": "sysadmin", "role": "admin", "email": "admin@example.com"},
}

# Support session: session_token -> authenticated user_id
SUPPORT_SESSIONS = {
    "sess-debug-aaa": "usr-001",
}

# Audit log (append-only in a real system)
AUDIT_LOG = []
