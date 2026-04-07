"""Authentication handler for test_creds_in_prod scenario -- VULNERABLE variant.

POST /login accepts the hardcoded test credentials in addition to
real user credentials. Any attacker who recognises the test account
name can authenticate without a valid user account.

Chain: POST /login -> config.TEST_PASSWORD accepted -> unauthorized session
"""
from flask import Blueprint, request, jsonify
import config

auth_bp = Blueprint("auth", __name__)


# vuln-code-snippet start chain_test_creds_vuln
@auth_bp.route("/login", methods=["POST"])
def login():
    """Authenticate a user, accepting hardcoded test credentials as valid."""
    username = request.json.get("username", "")
    password = request.json.get("password", "")
    if username == config.TEST_USERNAME and password == config.TEST_PASSWORD:  # vuln-code-snippet vuln-line chain_test_creds_vuln
        return jsonify({"authenticated": True, "role": "tester"})
    return jsonify({"authenticated": False}), 401
# vuln-code-snippet end chain_test_creds_vuln
