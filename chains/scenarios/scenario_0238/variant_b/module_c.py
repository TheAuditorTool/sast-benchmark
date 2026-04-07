import secrets
from flask import request, jsonify
from module_b import app, USERS

@app.route("/password-reset/request", methods=["POST"])
def request_reset():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 404
    token = secrets.token_urlsafe(32)
    user["reset_token"] = token
    return jsonify({"message": "Reset email sent if account exists"})
