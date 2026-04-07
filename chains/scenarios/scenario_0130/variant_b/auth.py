import functools
from flask import request, jsonify
from config import app, APP_SECRET
from debug import log_request

_USERS = {
    "alice": "alice-password",
    "admin": "admin-password",
}

@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    log_request("/login", data)
    username = data.get("username", "")
    password = data.get("password", "")
    if _USERS.get(username) != password:
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "token": APP_SECRET + "-" + username})

if __name__ == "__main__":
    app.run(port=5000)
