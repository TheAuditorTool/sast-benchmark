"""Authentication endpoints -- VULNERABLE variant.

This file is IDENTICAL between vuln/ and safe/ variants.
The login endpoint promotes the current session token to authenticated
without requesting a fresh token from session.py.  Whether fixation is
possible depends entirely on whether session.py regenerates the token
inside promote_session() -- it does not in the vuln variant.

Chain: pre-planted session token -> POST /login -> token promoted without
       regeneration -> attacker reuses same token (CWE-384)
"""
from flask import Flask, request, jsonify, make_response
from session import get_or_create_session, promote_session

app = Flask(__name__)

# Simulated user store
_USERS = {
    "alice": {"password": "hunter2", "role": "admin"},
    "bob":   {"password": "pass1234", "role": "user"},
}


# vuln-code-snippet start chain_no_regen_login_safe
@app.route("/login", methods=["POST"])
def login():
    """Authenticate the user and mark the current session as authenticated."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    existing_token = request.cookies.get("session_token", "")
    _, _ = get_or_create_session(existing_token)

    new_token = promote_session(existing_token, username, user["role"])  # vuln-code-snippet safe-line chain_no_regen_login_safe

    resp = make_response(jsonify({"status": "logged in", "user": username}))
    resp.set_cookie("session_token", new_token, httponly=True)
    return resp
# vuln-code-snippet end chain_no_regen_login_safe


@app.route("/logout", methods=["POST"])
def logout():
    """Clear the session cookie."""
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session_token")
    return resp
