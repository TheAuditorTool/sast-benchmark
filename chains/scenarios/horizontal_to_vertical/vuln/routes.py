"""HR profile routes -- VULNERABLE variant.

PATCH /profiles/<user_id> correctly enforces own-profile access but
applies all supplied fields without filtering, so an employee can
include role=manager in the patch body to escalate privileges.

Chain: user PATCHes own profile with role=manager -> all fields applied -> role escalated
"""
from flask import request, jsonify
from models import app, PROFILES
from auth import require_own_profile


@app.route("/profiles/<user_id>", methods=["GET"])
def get_profile(user_id):
    """Retrieve a user profile."""
    profile = PROFILES.get(user_id)
    if profile is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(profile)


# vuln-code-snippet start chain_horiz_to_vert_vuln
@app.route("/profiles/<user_id>", methods=["PATCH"])
@require_own_profile
def update_profile(user_id):
    """Update own profile. VULNERABLE: role field is not excluded from patch."""
    profile = PROFILES[user_id]
    data = request.get_json(force=True) or {}
    for key, value in data.items():
        profile[key] = value  # vuln-code-snippet vuln-line chain_horiz_to_vert_vuln
    return jsonify({"status": "updated", "profile": profile})
# vuln-code-snippet end chain_horiz_to_vert_vuln


if __name__ == "__main__":
    app.run(port=5000)
