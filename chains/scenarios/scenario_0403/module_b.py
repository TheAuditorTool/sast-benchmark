from flask import Blueprint, request, jsonify
import module_c

auth_bp = Blueprint("auth", __name__)

@auth_bp.route("/login", methods=["POST"])
def login():
    username = request.json.get("username", "")
    password = request.json.get("password", "")
    if username == config.ADMIN_USERNAME and password == config.ADMIN_PASSWORD:
        return jsonify({"authenticated": True, "role": "admin"})
    return jsonify({"authenticated": False}), 401
