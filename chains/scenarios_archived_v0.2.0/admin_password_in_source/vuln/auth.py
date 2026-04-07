"""Authentication handler for admin_password_in_source scenario -- VULNERABLE variant.

POST /login checks the supplied credentials against the hardcoded
config values. Because ADMIN_PASSWORD is a known constant, the check
always passes for any attacker who has read the source.

Chain: POST /login -> config.ADMIN_PASSWORD comparison -> admin session granted
"""
from flask import Blueprint, request, jsonify
import config

auth_bp = Blueprint("auth", __name__)


# vuln-code-snippet start chain_admin_pwd_source_vuln
@auth_bp.route("/login", methods=["POST"])
def login():
    """Authenticate a user against the hardcoded admin credentials."""
    username = request.json.get("username", "")
    password = request.json.get("password", "")
    if username == config.ADMIN_USERNAME and password == config.ADMIN_PASSWORD:  # vuln-code-snippet vuln-line chain_admin_pwd_source_vuln
        return jsonify({"authenticated": True, "role": "admin"})
    return jsonify({"authenticated": False}), 401
# vuln-code-snippet end chain_admin_pwd_source_vuln
