"""Versioned user management routes -- VULNERABLE variant.

/api/v2/users/<id>/disable enforces admin auth.
/api/v1/users/<id>/disable uses the legacy no-op decorator and is still mounted.

Chain: attacker uses v1 endpoint -> no auth -> user account disabled
"""
from flask import request, jsonify
from models import app, USERS
from auth import v1_require_admin, v2_require_admin


@app.route("/api/v2/users/<user_id>", methods=["GET"])
def get_user_v2(user_id):
    """Retrieve user details (v2)."""
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(user)


@app.route("/api/v2/users/<user_id>/disable", methods=["POST"])
@v2_require_admin
def disable_user_v2(user_id):
    """Disable a user account (v2, admin-only, authenticated)."""
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    user["notifications_enabled"] = False
    return jsonify({"status": "disabled", "user_id": user_id})


# vuln-code-snippet start chain_api_version_vuln
@app.route("/api/v1/users/<user_id>/disable", methods=["POST"])
@v1_require_admin
def disable_user_v1(user_id):
    """Disable a user account (v1). VULNERABLE: legacy route with no auth."""
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    user["notifications_enabled"] = False  # vuln-code-snippet vuln-line chain_api_version_vuln
    return jsonify({"status": "disabled", "user_id": user_id})
# vuln-code-snippet end chain_api_version_vuln


if __name__ == "__main__":
    app.run(port=5000)
