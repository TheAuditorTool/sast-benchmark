from flask import request, jsonify
from models import app, USERS
from auth import v1_require_admin, v2_require_admin

@app.route("/api/v2/users/<user_id>", methods=["GET"])
def get_user_v2(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(user)

@app.route("/api/v2/users/<user_id>/disable", methods=["POST"])
@v2_require_admin
def disable_user_v2(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    user["notifications_enabled"] = False
    return jsonify({"status": "disabled", "user_id": user_id})

# vuln-code-snippet start ChainScenario0145A
@app.route("/api/v1/users/<user_id>/disable", methods=["POST"])
@v1_require_admin
def disable_user_v1(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    user["notifications_enabled"] = False  # vuln-code-snippet target-line ChainScenario0145A
    return jsonify({"status": "disabled", "user_id": user_id})
# vuln-code-snippet end ChainScenario0145A

if __name__ == "__main__":
    app.run(port=5000)
