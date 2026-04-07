"""API token endpoints -- IDENTICAL between vuln/ and safe/ variants.

POST /register creates a user account and issues an API token.
POST /token/revoke revokes the current token.
Whether the token expires depends on session.py.

Chain: token issued at /register -> leaked -> replayed indefinitely (CWE-384)
"""
from flask import Flask, request, jsonify
from session import issue_api_token, revoke_api_token

app = Flask(__name__)

_USERS = {}


# vuln-code-snippet start chain_api_token_expire_safe
@app.route("/register", methods=["POST"])
def register():
    """Create a new user account and return a time-limited API token."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    if not username or not password:
        return jsonify({"error": "username and password required"}), 400
    if username in _USERS:
        return jsonify({"error": "User already exists"}), 409

    _USERS[username] = {"password": password, "role": "user"}
    token = issue_api_token(username, "user")  # vuln-code-snippet safe-line chain_api_token_expire_safe
    return jsonify({"status": "registered", "api_token": token}), 201
# vuln-code-snippet end chain_api_token_expire_safe


@app.route("/token/revoke", methods=["POST"])
def revoke():
    """Revoke the caller's API token."""
    auth_header = request.headers.get("Authorization", "")
    token = auth_header.removeprefix("Bearer ").strip()
    revoke_api_token(token)
    return jsonify({"status": "revoked"})
