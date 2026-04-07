"""CORS middleware -- SAFE variant.

Validates the incoming Origin against an explicit allowlist.  If the
origin is not on the list, no Access-Control-Allow-Origin header is set,
preventing cross-origin reads.  Credentialed requests are only permitted
for explicitly allowed origins.

CWE-200: Fixed by validating Origin against an allowlist instead of reflecting it.
Chain: Cross-origin request from evil.example.com -> origin not in allowlist -> CORS blocked
"""
from flask import request, jsonify
from config import app, SESSIONS

_ALLOWED_ORIGINS = {"https://app.example.com", "https://www.example.com"}


# vuln-code-snippet start chain_cors_cred_leak_safe
@app.after_request
def cors_headers(response):
    """Set CORS headers only for allowed origins.

    SAFE: Origin is checked against _ALLOWED_ORIGINS; unknown origins
    receive no Access-Control-Allow-Origin header.
    """
    origin = request.headers.get("Origin", "")
    if origin in _ALLOWED_ORIGINS:  # vuln-code-snippet safe-line chain_cors_cred_leak_safe
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
    return response
# vuln-code-snippet end chain_cors_cred_leak_safe


@app.route("/api/token")
def get_token():
    """Return the current session's API token."""
    session_id = request.cookies.get("session_id", "")
    session = SESSIONS.get(session_id)
    if session is None:
        return jsonify({"error": "Not authenticated"}), 401
    return jsonify({"api_token": session["api_token"]})
