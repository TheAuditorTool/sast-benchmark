from flask import jsonify
from config import app, USERS

_SAFE_FIELDS = {"username", "email"}

# vuln-code-snippet start ChainScenario0233A
@app.route("/api/v1/users/<user_id>")
def get_user_v1(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify({k: v for k, v in user.items() if k in _SAFE_FIELDS})  # vuln-code-snippet target-line ChainScenario0233A
# vuln-code-snippet end ChainScenario0233A

@app.route("/api/v2/users/<user_id>")
def get_user_v2(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify({"username": user["username"], "email": user["email"]})
