from flask import Flask, request, jsonify
from session import issue_api_token, revoke_api_token

app = Flask(__name__)

_USERS = {}

# vuln-code-snippet start ChainScenario0014B
@app.route("/register", methods=["POST"])
def register():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    if not username or not password:
        return jsonify({"error": "username and password required"}), 400
    if username in _USERS:
        return jsonify({"error": "User already exists"}), 409

    _USERS[username] = {"password": password, "role": "user"}
    token = issue_api_token(username, "user")  # vuln-code-snippet target-line ChainScenario0014B
    return jsonify({"status": "registered", "api_token": token}), 201
# vuln-code-snippet end ChainScenario0014B

@app.route("/token/revoke", methods=["POST"])
def revoke():
    auth_header = request.headers.get("Authorization", "")
    token = auth_header.removeprefix("Bearer ").strip()
    revoke_api_token(token)
    return jsonify({"status": "revoked"})
