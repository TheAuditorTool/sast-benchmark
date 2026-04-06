"""Login endpoint -- VULNERABLE variant.

Returns different error messages for unknown username vs wrong password,
allowing an attacker to enumerate valid usernames through the error text.

CWE-200: Exposure of Sensitive Information via distinguishable error messages.
Chain:
  1. POST /login with unknown username returns "User not found".
  2. POST /login with known username + wrong password returns "Wrong password".
  3. Attacker iterates usernames until they get "Wrong password", confirming existence.
"""
from flask import request, jsonify
from config import app, USERS


# vuln-code-snippet start chain_username_enum_vuln
@app.route("/login", methods=["POST"])
def login():
    """Authenticate a user.

    VULNERABLE: distinct error messages reveal whether the username exists.
    """
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 401  # vuln-code-snippet vuln-line chain_username_enum_vuln
    if user["password"] != password:
        return jsonify({"error": "Wrong password"}), 401
    return jsonify({"status": "ok", "role": user["role"]})
# vuln-code-snippet end chain_username_enum_vuln
