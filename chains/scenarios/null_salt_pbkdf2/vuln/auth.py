"""Auth endpoints for null_salt_pbkdf2 scenario.

POST /register hashes and stores a password.
POST /login verifies the password and returns a session indicator.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, jsonify
from crypto import hash_password, verify_password

auth_bp = Blueprint("auth", __name__)

_DB: dict = {}


@auth_bp.route("/register", methods=["POST"])
def register():
    """Store a hashed password for the given username."""
    data = request.json or {}
    username = data.get("username", "")
    password = data.get("password", "")
    if not username or not password:
        return jsonify({"error": "username and password required"}), 400
    _DB[username] = hash_password(password)
    return jsonify({"status": "registered"})


# vuln-code-snippet start chain_null_salt_vuln
@auth_bp.route("/login", methods=["POST"])
def login():
    """Verify password and return session status."""
    data = request.json or {}
    username = data.get("username", "")
    password = data.get("password", "")
    stored = _DB.get(username)
    if not stored or not verify_password(password, stored):
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "user": username})  # vuln-code-snippet vuln-line chain_null_salt_vuln
# vuln-code-snippet end chain_null_salt_vuln
