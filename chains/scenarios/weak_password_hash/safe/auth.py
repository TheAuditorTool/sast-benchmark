"""Auth endpoints for weak_password_hash scenario -- SAFE variant.

POST /register stores a PBKDF2-hashed password.
POST /login verifies the password.

This file is IDENTICAL between vuln/ and safe/ variants (only
crypto.py changes).
"""
from flask import Blueprint, request, jsonify
from crypto import hash_password, verify_password

auth_bp = Blueprint("auth", __name__)

_DB: dict = {}


@auth_bp.route("/register", methods=["POST"])
def register():
    """Hash and store a PBKDF2 password for the supplied username."""
    data = request.json or {}
    user = data.get("username", "")
    pw = data.get("password", "")
    if not user or not pw:
        return jsonify({"error": "username and password required"}), 400
    _DB[user] = hash_password(pw)
    return jsonify({"status": "registered"})


# vuln-code-snippet start chain_weak_password_hash_safe
@auth_bp.route("/login", methods=["POST"])
def login():
    """Verify PBKDF2 password and return session status."""
    data = request.json or {}
    user = data.get("username", "")
    pw = data.get("password", "")
    stored = _DB.get(user)
    if not stored or not verify_password(pw, stored):
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "user": user})  # vuln-code-snippet safe-line chain_weak_password_hash_safe
# vuln-code-snippet end chain_weak_password_hash_safe
