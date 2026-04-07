"""Password comparison helper -- SAFE variant.

Uses hmac.compare_digest for constant-time comparison, eliminating the
timing side-channel that would allow password enumeration.

CWE-200: Fixed by using constant-time comparison.
Chain: POST /login with varying passwords -> uniform response time -> no timing oracle
"""
import hmac
from flask import request, jsonify
from config import app, USERS


# vuln-code-snippet start chain_timing_oracle_safe
def check_password(stored, supplied):
    """Compare stored and supplied passwords using constant-time comparison.

    SAFE: hmac.compare_digest runs in constant time regardless of where
    the strings first differ, removing the timing side-channel.
    """
    return hmac.compare_digest(stored, supplied)  # vuln-code-snippet safe-line chain_timing_oracle_safe
# vuln-code-snippet end chain_timing_oracle_safe


@app.route("/login", methods=["POST"])
def login():
    """Authenticate a user by username and password."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = USERS.get(username)
    if user is None or not check_password(user["password"], password):
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "role": user["role"]})
