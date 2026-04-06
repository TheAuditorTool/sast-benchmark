"""Authentication endpoints -- IDENTICAL between vuln/ and safe/ variants.

POST /login creates a new session without invalidating existing ones.
Whether old sessions are evicted when a new login occurs depends on session.py.

Chain: session stolen -> attacker and victim both active -> no eviction ->
       attacker maintains access after victim re-authenticates (CWE-384)
"""
from flask import Flask, request, jsonify, make_response
from session import create_session

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_a", "role": "admin"},
    "bob":   {"password": "pw_b", "role": "user"},
}


# vuln-code-snippet start chain_concurrent_session_safe
@app.route("/login", methods=["POST"])
def login():
    """Authenticate and issue a new session, invalidating all prior sessions."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    client_ip = request.remote_addr or "unknown"
    token = create_session(username, user["role"], client_ip)  # vuln-code-snippet safe-line chain_concurrent_session_safe

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("session", token, httponly=True)
    return resp
# vuln-code-snippet end chain_concurrent_session_safe


@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session")
    return resp
