from flask import Blueprint, request, jsonify
from tokens import make_token, verify_token

auth_bp = Blueprint("auth", __name__)

@auth_bp.route("/login", methods=["POST"])
def login():
    user_id = request.json.get("user_id", "anonymous")
    token = make_token(user_id)
    return jsonify({"token": token})

# vuln-code-snippet start ChainScenario0204B
@auth_bp.route("/protected")
def protected():
    token = request.headers.get("X-Token", "")
    user_id = verify_token(token)
    if not user_id:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"data": "secret", "user": user_id})  # vuln-code-snippet target-line ChainScenario0204B
# vuln-code-snippet end ChainScenario0204B
