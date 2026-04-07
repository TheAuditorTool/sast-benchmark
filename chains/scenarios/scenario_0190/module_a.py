import functools
from flask import request, jsonify
from module_b import app, USERS
from module_c import request_reset  

@app.route("/password-reset/confirm", methods=["POST"])
def confirm_reset():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    token = data.get("token", "")
    new_password = data.get("new_password", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 404
    if not token or user.get("reset_token") != token:
        return jsonify({"error": "Invalid or expired token"}), 401
    user["password"] = new_password
    user["reset_token"] = None
    return jsonify({"status": "password updated"})

if __name__ == "__main__":
    app.run(port=5000)
