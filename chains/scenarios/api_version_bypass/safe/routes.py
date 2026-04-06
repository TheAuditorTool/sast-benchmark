"""Versioned user management routes -- SAFE variant.

/api/v1/users/<id>/disable now uses v1_require_admin which applies
the same full admin check as v2, closing the version bypass.

Chain: attacker uses v1 endpoint -> same auth as v2 applied -> 401/403
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


# vuln-code-snippet start chain_api_version_safe
@app.route("/api/v1/users/<user_id>/disable", methods=["POST"])
@v1_require_admin
def disable_user_v1(user_id):
    """Disable a user account (v1). SAFE: same admin auth as v2 now applied."""
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    user["notifications_enabled"] = False  # vuln-code-snippet safe-line chain_api_version_safe
    return jsonify({"status": "disabled", "user_id": user_id})
# vuln-code-snippet end chain_api_version_safe


if __name__ == "__main__":
    app.run(port=5000)
