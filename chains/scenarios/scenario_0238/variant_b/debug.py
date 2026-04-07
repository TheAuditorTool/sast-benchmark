import secrets
from flask import request, jsonify
from config import app, USERS

# vuln-code-snippet start ChainScenario0238B
@app.route("/password-reset/request", methods=["POST"])
def request_reset():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 404
    token = secrets.token_urlsafe(32)
    user["reset_token"] = token
    return jsonify({"message": "Reset email sent if account exists"})  # vuln-code-snippet target-line ChainScenario0238B
# vuln-code-snippet end ChainScenario0238B
