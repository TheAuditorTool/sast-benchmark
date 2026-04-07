from flask import request, jsonify
from module_b import app, PROFILES
from module_a import require_own_profile

@app.route("/profiles/<user_id>", methods=["GET"])
def get_profile(user_id):
    profile = PROFILES.get(user_id)
    if profile is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(profile)

@app.route("/profiles/<user_id>", methods=["PATCH"])
@require_own_profile
def update_profile(user_id):
    profile = PROFILES[user_id]
    data = request.get_json(force=True) or {}
    for key, value in data.items():
        profile[key] = value
    return jsonify({"status": "updated", "profile": profile})

if __name__ == "__main__":
    app.run(port=5000)
