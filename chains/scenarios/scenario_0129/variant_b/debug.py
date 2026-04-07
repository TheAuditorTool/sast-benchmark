from flask import request, jsonify
from config import app, USERS

# vuln-code-snippet start ChainScenario0129B
@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 401  # vuln-code-snippet target-line ChainScenario0129B
    if user["password"] != password:
        return jsonify({"error": "Wrong password"}), 401
    return jsonify({"status": "ok", "role": user["role"]})
# vuln-code-snippet end ChainScenario0129B
