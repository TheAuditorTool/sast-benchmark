"""HR profile routes -- SAFE variant.

PATCH /profiles/<user_id> now filters the patch body to only allow
fields in SELF_EDITABLE_FIELDS, preventing role elevation.

Chain: user PATCHes own profile with role=manager -> role key filtered out -> chain broken
"""
from flask import request, jsonify
from models import app, PROFILES, SELF_EDITABLE_FIELDS
from auth import require_own_profile


@app.route("/profiles/<user_id>", methods=["GET"])
def get_profile(user_id):
    """Retrieve a user profile."""
    profile = PROFILES.get(user_id)
    if profile is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(profile)


# vuln-code-snippet start chain_horiz_to_vert_safe
@app.route("/profiles/<user_id>", methods=["PATCH"])
@require_own_profile
def update_profile(user_id):
    """Update own profile. SAFE: only allowlisted fields are applied."""
    profile = PROFILES[user_id]
    data = request.get_json(force=True) or {}
    for key, value in data.items():
        if key in SELF_EDITABLE_FIELDS:
            profile[key] = value  # vuln-code-snippet safe-line chain_horiz_to_vert_safe
    return jsonify({"status": "updated", "profile": profile})
# vuln-code-snippet end chain_horiz_to_vert_safe


if __name__ == "__main__":
    app.run(port=5000)
