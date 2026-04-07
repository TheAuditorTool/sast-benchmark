import hmac
from flask import request, jsonify
from config import app, USERS

# vuln-code-snippet start ChainScenario0068B
def check_password(stored, supplied):
    return hmac.compare_digest(stored, supplied)  # vuln-code-snippet target-line ChainScenario0068B
# vuln-code-snippet end ChainScenario0068B

@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = USERS.get(username)
    if user is None or not check_password(user["password"], password):
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "role": user["role"]})
