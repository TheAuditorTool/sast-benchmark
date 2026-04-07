import secrets
from flask import request, jsonify
from config import app, USERS

# vuln-code-snippet start ChainScenario0238A
@app.route("/password-reset/request", methods=["POST"])
def request_reset():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 404
    token = secrets.token_urlsafe(32)
    user["reset_token"] = token
    return jsonify({"message": "Reset initiated", "token": token})  # vuln-code-snippet target-line ChainScenario0238A
# vuln-code-snippet end ChainScenario0238A
