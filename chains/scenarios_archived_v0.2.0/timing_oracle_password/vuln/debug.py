"""Password comparison helper -- VULNERABLE variant.

Uses a plain string equality check to verify passwords, which exits on
the first differing byte and creates a timing side-channel that allows
an attacker to enumerate the correct password character by character.

CWE-200: Timing side-channel leaks password information.
Chain:
  1. POST /login with varying passwords; measure response latency.
  2. Longer response times indicate a longer common prefix, narrowing the search.
  3. Attacker reconstructs the password and logs in.
"""
from flask import request, jsonify
from config import app, USERS


# vuln-code-snippet start chain_timing_oracle_vuln
def check_password(stored, supplied):
    """Compare stored and supplied passwords.

    VULNERABLE: plain == comparison short-circuits on first mismatch,
    leaking password length and content via timing.
    """
    return stored == supplied  # vuln-code-snippet vuln-line chain_timing_oracle_vuln
# vuln-code-snippet end chain_timing_oracle_vuln


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
