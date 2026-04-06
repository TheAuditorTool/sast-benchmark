"""Session-gated admin action.

This file is IDENTICAL between vuln/ and safe/ variants.

Uses the api_token from the session to authorise admin actions.  Because
debug.py's CORS misconfiguration allows cross-origin reads, an attacker
can steal the api_token from a victim's browser.

CWE-200: CORS credential reflection enables cross-origin token theft leading to account takeover.
Chain: CORS reflects origin + allow-credentials -> attacker reads api_token -> admin action executed
"""
import functools
from flask import request, jsonify
from config import app, SESSIONS
from debug import cors_headers, get_token  # noqa: F401 - registers routes and hook


def _require_api_token(f):
    """Require X-Api-Token header matching a session token."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get("X-Api-Token", "")
        session = next(
            (s for s in SESSIONS.values() if s["api_token"] == token),
            None,
        )
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_session = session
        return f(*args, **kwargs)
    return decorated


@app.route("/api/admin/wipe", methods=["POST"])
@_require_api_token
def admin_wipe():
    """Wipe all non-admin sessions (admin only)."""
    if request.current_session.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    keys = [k for k, v in SESSIONS.items() if v["role"] != "admin"]
    for k in keys:
        del SESSIONS[k]
    return jsonify({"status": "wiped", "count": len(keys)})


if __name__ == "__main__":
    app.run(port=5000)
