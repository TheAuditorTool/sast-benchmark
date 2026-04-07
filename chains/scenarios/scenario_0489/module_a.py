from flask import Blueprint, request, jsonify
from module_c import make_token, verify_token

auth_bp = Blueprint("auth", __name__)

@auth_bp.route("/login", methods=["POST"])
def login():
    user = (request.json or {}).get("user", "guest")
    return jsonify({"token": make_token({"sub": user, "role": "user"})})

@auth_bp.route("/protected")
def protected():
    token = request.headers.get("Authorization", "").removeprefix("Bearer ")
    claims = verify_token(token)
    if not claims:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"data": "protected", "claims": claims})
