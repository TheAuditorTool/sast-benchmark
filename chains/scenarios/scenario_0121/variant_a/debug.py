from flask import request, jsonify
from config import app, USERS

# vuln-code-snippet start ChainScenario0121A
@app.after_request
def add_server_header(response):
    response.headers["Server"] = "App"  # vuln-code-snippet target-line ChainScenario0121A
    return response
# vuln-code-snippet end ChainScenario0121A

@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "role": user["role"]})
