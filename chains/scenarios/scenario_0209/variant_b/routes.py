from flask import request, jsonify
from models import app, PROFILES, SELF_EDITABLE_FIELDS
from auth import require_own_profile

@app.route("/profiles/<user_id>", methods=["GET"])
def get_profile(user_id):
    profile = PROFILES.get(user_id)
    if profile is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(profile)

# vuln-code-snippet start ChainScenario0209B
@app.route("/profiles/<user_id>", methods=["PATCH"])
@require_own_profile
def update_profile(user_id):
    profile = PROFILES[user_id]
    data = request.get_json(force=True) or {}
    for key, value in data.items():
        if key in SELF_EDITABLE_FIELDS:
            profile[key] = value  # vuln-code-snippet target-line ChainScenario0209B
    return jsonify({"status": "updated", "profile": profile})
# vuln-code-snippet end ChainScenario0209B

if __name__ == "__main__":
    app.run(port=5000)
