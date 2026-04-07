"""API v1 (legacy) user endpoint -- VULNERABLE variant.

The older v1 API returns the full user record including password_hash,
which was excluded when v2 was introduced.  The v1 route still exists and
is reachable.

CWE-200: Exposure of Sensitive Information
Chain:
  1. GET /api/v1/users/<id> returns password_hash and api_token.
  2. An attacker uses the harvested token to authenticate as that user.
"""
from flask import jsonify
from config import app, USERS


# vuln-code-snippet start chain_api_version_leak_vuln
@app.route("/api/v1/users/<user_id>")
def get_user_v1(user_id):
    """Return a full user record including password_hash (legacy endpoint).

    VULNERABLE: password_hash and api_token are included in the response body.
    """
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(user)  # vuln-code-snippet vuln-line chain_api_version_leak_vuln
# vuln-code-snippet end chain_api_version_leak_vuln


@app.route("/api/v2/users/<user_id>")
def get_user_v2(user_id):
    """Return a redacted user record (current endpoint)."""
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify({"username": user["username"], "email": user["email"]})
