"""API v1 (legacy) user endpoint -- SAFE variant.

The v1 endpoint no longer returns password_hash or api_token.  Both fields
are excluded before the response is serialised.

CWE-200: Fixed by stripping sensitive fields from the legacy API response.
Chain: GET /api/v1/users/<id> -> password_hash and api_token omitted -> no token leaked
"""
from flask import jsonify
from config import app, USERS

_SAFE_FIELDS = {"username", "email"}


# vuln-code-snippet start chain_api_version_leak_safe
@app.route("/api/v1/users/<user_id>")
def get_user_v1(user_id):
    """Return a redacted user record from the legacy endpoint."""
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify({k: v for k, v in user.items() if k in _SAFE_FIELDS})  # vuln-code-snippet safe-line chain_api_version_leak_safe
# vuln-code-snippet end chain_api_version_leak_safe


@app.route("/api/v2/users/<user_id>")
def get_user_v2(user_id):
    """Return a redacted user record (current endpoint)."""
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify({"username": user["username"], "email": user["email"]})
