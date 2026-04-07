from flask import request, jsonify
from models import app, PROFILES
from auth import require_own_profile

@app.route("/profiles/<user_id>", methods=["GET"])
def get_profile(user_id):
    profile = PROFILES.get(user_id)
    if profile is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(profile)

# vuln-code-snippet start ChainScenario0209A
@app.route("/profiles/<user_id>", methods=["PATCH"])
@require_own_profile
def update_profile(user_id):
    profile = PROFILES[user_id]
    data = request.get_json(force=True) or {}
    for key, value in data.items():
        profile[key] = value  # vuln-code-snippet target-line ChainScenario0209A
    return jsonify({"status": "updated", "profile": profile})
# vuln-code-snippet end ChainScenario0209A

if __name__ == "__main__":
    app.run(port=5000)
