"""Authentication endpoints -- IDENTICAL between vuln/ and safe/ variants.

A pre-login anonymous session is created on GET /init (simulating the HTTP
landing page).  POST /login promotes that session to authenticated.  Whether
the cookie is Secure (HTTPS-only) and whether the token is regenerated depends
on session.py.

Chain: /init over HTTP, no Secure flag -> attacker reads cookie -> /login
       promotes same token -> attacker replays it (CWE-384)
"""
from flask import Flask, request, jsonify, make_response
from session import create_session, promote_session, get_secure_flag

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_a", "role": "admin"},
    "eve":   {"password": "pw_e", "role": "user"},
}


@app.route("/init")
def init():
    """Create an anonymous pre-login session (simulates HTTP landing page)."""
    token = create_session()
    resp = make_response(jsonify({"status": "session started"}))
    resp.set_cookie("session", token, httponly=True, secure=get_secure_flag())
    return resp


# vuln-code-snippet start chain_cross_protocol_safe
@app.route("/login", methods=["POST"])
def login():
    """Authenticate and promote the existing session."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    existing = request.cookies.get("session", "")
    token = promote_session(existing, username, user["role"])  # vuln-code-snippet safe-line chain_cross_protocol_safe

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("session", token, httponly=True, secure=get_secure_flag())
    return resp
# vuln-code-snippet end chain_cross_protocol_safe


@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session")
    return resp
