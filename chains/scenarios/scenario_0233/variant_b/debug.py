from flask import jsonify
from config import app, USERS

# vuln-code-snippet start ChainScenario0233B
@app.route("/api/v1/users/<user_id>")
def get_user_v1(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(user)  # vuln-code-snippet target-line ChainScenario0233B
# vuln-code-snippet end ChainScenario0233B

@app.route("/api/v2/users/<user_id>")
def get_user_v2(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify({"username": user["username"], "email": user["email"]})
