"""Authentication handler for admin_password_in_source scenario -- SAFE variant.

POST /login checks credentials against values loaded from environment
variables. No credential is present in the source, breaking the chain.

Chain broken: config.ADMIN_PASSWORD from env -> no hardcoded value -> no unauthorized access
"""
from flask import Blueprint, request, jsonify
import config

auth_bp = Blueprint("auth", __name__)


# vuln-code-snippet start chain_admin_pwd_source_safe
@auth_bp.route("/login", methods=["POST"])
def login():
    """Authenticate a user against environment-sourced admin credentials."""
    username = request.json.get("username", "")
    password = request.json.get("password", "")
    if username == config.ADMIN_USERNAME and password == config.ADMIN_PASSWORD:  # vuln-code-snippet safe-line chain_admin_pwd_source_safe
        return jsonify({"authenticated": True, "role": "admin"})
    return jsonify({"authenticated": False}), 401
# vuln-code-snippet end chain_admin_pwd_source_safe
