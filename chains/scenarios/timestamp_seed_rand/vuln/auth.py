"""Auth endpoints for timestamp_seed_rand scenario.

POST /login issues a session token.
GET  /session verifies a session token.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, jsonify
from crypto import generate_token

auth_bp = Blueprint("auth", __name__)

_SESSIONS: dict = {}


@auth_bp.route("/login", methods=["POST"])
def login():
    """Issue a session token for the supplied username."""
    username = (request.json or {}).get("username", "guest")
    token = generate_token()
    _SESSIONS[token] = username
    return jsonify({"token": token})


# vuln-code-snippet start chain_timestamp_seed_vuln
@auth_bp.route("/session")
def session():
    """Verify a session token and return the associated username."""
    token = request.headers.get("X-Session-Token", "")
    username = _SESSIONS.get(token)
    if not username:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"user": username})  # vuln-code-snippet vuln-line chain_timestamp_seed_vuln
# vuln-code-snippet end chain_timestamp_seed_vuln
