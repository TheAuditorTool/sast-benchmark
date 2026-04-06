"""Login endpoint -- SAFE variant.

Returns a single generic error message regardless of whether the username
exists or the password is wrong, preventing username enumeration.

CWE-200: Fixed by using a uniform error message for all authentication failures.
Chain: POST /login -> uniform "Invalid credentials" -> username existence not revealed
"""
from flask import request, jsonify
from config import app, USERS


# vuln-code-snippet start chain_username_enum_safe
@app.route("/login", methods=["POST"])
def login():
    """Authenticate a user with a uniform error message.

    SAFE: the same error is returned whether the username is unknown or
    the password is incorrect; the attacker cannot distinguish the two cases.
    """
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401  # vuln-code-snippet safe-line chain_username_enum_safe
    return jsonify({"status": "ok", "role": user["role"]})
# vuln-code-snippet end chain_username_enum_safe
