from flask import request, jsonify
from config import app, USERS

# vuln-code-snippet start ChainScenario0129A
@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401  # vuln-code-snippet target-line ChainScenario0129A
    return jsonify({"status": "ok", "role": user["role"]})
# vuln-code-snippet end ChainScenario0129A
