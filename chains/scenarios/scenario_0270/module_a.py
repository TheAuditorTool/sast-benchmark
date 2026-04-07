from flask import Blueprint, request, jsonify
from module_c import generate_token

auth_bp = Blueprint("auth", __name__)

_SESSIONS: dict = {}

@auth_bp.route("/login", methods=["POST"])
def login():
    username = (request.json or {}).get("username", "guest")
    token = generate_token()
    _SESSIONS[token] = username
    return jsonify({"token": token})

@auth_bp.route("/session")
def session():
    token = request.headers.get("X-Session-Token", "")
    username = _SESSIONS.get(token)
    if not username:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"user": username})
