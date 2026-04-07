from flask import request, jsonify
from module_b import app, USERS

@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 401
    if user["password"] != password:
        return jsonify({"error": "Wrong password"}), 401
    return jsonify({"status": "ok", "role": user["role"]})
