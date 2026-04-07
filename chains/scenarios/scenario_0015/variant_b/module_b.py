from flask import Flask

app = Flask(__name__)
app.secret_key = "support-tool-secret"

USERS = {
    "usr-001": {"username": "alice", "role": "user", "email": "alice@example.com"},
    "usr-002": {"username": "bob", "role": "user", "email": "bob@example.com"},
    "adm-001": {"username": "sysadmin", "role": "admin", "email": "admin@example.com"},
}

SUPPORT_SESSIONS = {
    "sess-debug-aaa": "usr-001",
}

AUDIT_LOG = []
