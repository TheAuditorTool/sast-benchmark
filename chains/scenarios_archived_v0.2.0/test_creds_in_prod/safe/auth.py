"""Authentication handler for test_creds_in_prod scenario -- SAFE variant.

POST /login authenticates against the standard user store only.
There is no hardcoded test account; all credentials must come from
the configured identity provider.

Chain broken: no hardcoded test credentials -> no bypass login -> no unauthorized access
"""
from flask import Blueprint, request, jsonify

auth_bp = Blueprint("auth", __name__)

_USER_STORE = {}


# vuln-code-snippet start chain_test_creds_safe
@auth_bp.route("/login", methods=["POST"])
def login():
    """Authenticate a user against the standard user store with no hardcoded bypass."""
    username = request.json.get("username", "")
    password = request.json.get("password", "")
    stored = _USER_STORE.get(username)  # vuln-code-snippet safe-line chain_test_creds_safe
    if stored and stored == password:
        return jsonify({"authenticated": True, "role": "user"})
    return jsonify({"authenticated": False}), 401
# vuln-code-snippet end chain_test_creds_safe
