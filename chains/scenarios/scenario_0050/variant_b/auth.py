from flask import Blueprint, request, jsonify
import config

auth_bp = Blueprint("auth", __name__)

# vuln-code-snippet start ChainScenario0050B
@auth_bp.route("/login", methods=["POST"])
def login():
    username = request.json.get("username", "")
    password = request.json.get("password", "")
    if username == config.ADMIN_USERNAME and password == config.ADMIN_PASSWORD:  # vuln-code-snippet target-line ChainScenario0050B
        return jsonify({"authenticated": True, "role": "admin"})
    return jsonify({"authenticated": False}), 401
# vuln-code-snippet end ChainScenario0050B
