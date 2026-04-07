from flask import Blueprint, request, jsonify
from module_c import encrypt_claims, decrypt_claims

auth_bp = Blueprint("auth", __name__)

@auth_bp.route("/token", methods=["POST"])
def issue():
    claims = request.json or {"role": "user"}
    return jsonify({"token": encrypt_claims(claims)})

@auth_bp.route("/protected")
def protected():
    token = request.headers.get("X-Token", "")
    claims = decrypt_claims(token)
    if not claims:
        return jsonify({"error": "Forbidden"}), 403
    if claims.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"secret": "admin data"})
