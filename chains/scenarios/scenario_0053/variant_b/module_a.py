from flask import Blueprint, request, jsonify
from module_c import hash_password, verify_password

auth_bp = Blueprint("auth", __name__)

_DB: dict = {}

@auth_bp.route("/register", methods=["POST"])
def register():
    data = request.json or {}
    user = data.get("username", "")
    pw = data.get("password", "")
    if not user or not pw:
        return jsonify({"error": "username and password required"}), 400
    _DB[user] = hash_password(pw)
    return jsonify({"status": "registered"})

@auth_bp.route("/login", methods=["POST"])
def login():
    data = request.json or {}
    user = data.get("username", "")
    pw = data.get("password", "")
    stored = _DB.get(user)
    if not stored or not verify_password(pw, stored):
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "user": user})
