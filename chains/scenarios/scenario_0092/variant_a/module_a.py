from flask import Blueprint, request, jsonify
from module_c import make_token, verify_token

auth_bp = Blueprint("auth", __name__)

@auth_bp.route("/login", methods=["POST"])
def login():
    user_id = request.json.get("user_id", "anonymous")
    token = make_token({"sub": user_id, "role": "user"})
    return jsonify({"token": token})

@auth_bp.route("/protected")
def protected():
    token = request.headers.get("Authorization", "").removeprefix("Bearer ")
    claims = verify_token(token)
    if not claims:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"data": "secret", "claims": claims})
