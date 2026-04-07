"""Authentication endpoints -- IDENTICAL between vuln/ and safe/ variants.

The /remember endpoint exchanges a remember-me token for a new session.
Whether the remember-me token is rotated on use depends on session_store.py.

Chain: remember-me token stolen -> replayed via /remember -> session issued
       without token rotation -> token usable indefinitely (CWE-384)
"""
from flask import Flask, request, jsonify, make_response
from session_store import issue_remember_token, restore_session_from_remember

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_alice", "role": "admin"},
    "bob":   {"password": "pw_bob",   "role": "user"},
}


@app.route("/login", methods=["POST"])
def login():
    """Authenticate and optionally issue a remember-me token."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    remember = data.get("remember", False)

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    resp = make_response(jsonify({"status": "logged in"}))
    if remember:
        rtoken = issue_remember_token(username, user["role"])
        resp.set_cookie("remember_token", rtoken, httponly=True, max_age=30 * 86400)
    return resp


# vuln-code-snippet start chain_persistent_no_rotate_vuln
@app.route("/remember", methods=["POST"])
def remember():
    """Restore a session from the remember-me cookie."""
    rtoken = request.cookies.get("remember_token", "")
    session_token, new_rtoken = restore_session_from_remember(rtoken)  # vuln-code-snippet vuln-line chain_persistent_no_rotate_vuln
    if session_token is None:
        return jsonify({"error": "Invalid or expired remember token"}), 401

    resp = make_response(jsonify({"status": "session restored"}))
    resp.set_cookie("session_token", session_token, httponly=True)
    resp.set_cookie("remember_token", new_rtoken, httponly=True, max_age=30 * 86400)
    return resp
# vuln-code-snippet end chain_persistent_no_rotate_vuln


@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session_token")
    resp.delete_cookie("remember_token")
    return resp
