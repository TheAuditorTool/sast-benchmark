"""MFA enforcement module -- VULNERABLE variant.

MFA is only enforced when the request originates from the web UI, detected
by the presence of an 'X-Web-UI' header. API requests bypass MFA entirely.
An attacker can authenticate with just a username/password, omit the
X-Web-UI header, and reach protected endpoints without completing MFA.

Chain: API call (no X-Web-UI) -> MFA skipped -> valid session -> SQLi on /finance/transactions
Individual findings: MFA bypass (high) + SQLi (medium)
Chain finding: unauthenticated SQLi bypassing MFA (critical)
"""
import secrets
import functools
from flask import request, jsonify

# Simulated user store: username -> {password, mfa_secret, mfa_enabled}
_users = {
    "analyst": {"password": "p@ssw0rd!", "mfa_enabled": True},
}

# Sessions pending MFA: {token: username}
_pending_mfa = {}
# Fully authenticated sessions: {token: username}
_sessions = {}


def login():
    """Handle POST /auth/login. Returns a full session or MFA challenge."""
    data = request.get_json(silent=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = _users.get(username)
    if not user or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    from_web_ui = request.headers.get("X-Web-UI", "") == "1"
    if user["mfa_enabled"] and from_web_ui:
        # Web UI path: issue a pending token and require MFA step
        pending_token = secrets.token_hex(16)
        _pending_mfa[pending_token] = username
        return jsonify({"mfa_required": True, "pending_token": pending_token}), 202

    # VULNERABLE: API requests skip MFA and get a full session immediately
    session_token = secrets.token_hex(32)
    _sessions[session_token] = username
    return jsonify({"token": session_token})


def complete_mfa():
    """Handle POST /auth/mfa -- complete the MFA step."""
    data = request.get_json(silent=True) or {}
    pending_token = data.get("pending_token", "")
    mfa_code = data.get("code", "")
    username = _pending_mfa.pop(pending_token, None)
    if not username:
        return jsonify({"error": "Invalid or expired MFA challenge"}), 401
    # Simplified: accept any 6-digit code (real impl would verify TOTP)
    if len(mfa_code) != 6 or not mfa_code.isdigit():
        return jsonify({"error": "Invalid MFA code"}), 401
    session_token = secrets.token_hex(32)
    _sessions[session_token] = username
    return jsonify({"token": session_token})


def require_auth(f):
    """Decorator that validates a fully authenticated session token."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return jsonify({"error": "Authentication required"}), 401
        token = auth_header[7:]
        if token not in _sessions:
            return jsonify({"error": "Invalid or expired session"}), 401
        request.current_user = _sessions[token]
        return f(*args, **kwargs)
    return decorated
