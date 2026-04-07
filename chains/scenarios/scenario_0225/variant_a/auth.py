from flask import Flask, request, jsonify
from session import register_device, login_device

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_a", "role": "admin"},
    "bob":   {"password": "pw_b", "role": "user"},
}

@app.route("/device/register", methods=["POST"])
def register():
    data = request.get_json(force=True) or {}
    device_id = data.get("device_id", "")
    if not device_id:
        return jsonify({"error": "device_id required"}), 400

    token = register_device(device_id)
    return jsonify({"token": token})

# vuln-code-snippet start ChainScenario0225A
@app.route("/device/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    auth_header = request.headers.get("Authorization", "")
    token = auth_header.removeprefix("Bearer ").strip()

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    result_token = login_device(token, username)  # vuln-code-snippet target-line ChainScenario0225A
    if result_token is None:
        return jsonify({"error": "Invalid device token"}), 401

    return jsonify({"status": "logged in", "token": result_token})
# vuln-code-snippet end ChainScenario0225A

@app.route("/device/logout", methods=["POST"])
def logout():
    return jsonify({"status": "logged out"})
