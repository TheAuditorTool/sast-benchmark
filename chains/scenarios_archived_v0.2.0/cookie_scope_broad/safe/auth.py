"""Authentication endpoints -- IDENTICAL between vuln/ and safe/ variants.

Sets the session cookie using the domain returned by session.get_cookie_domain().
Whether the scope is restricted to the exact host or broadcast to all subdomains
depends entirely on what session.py returns from that function.

Chain: broad domain cookie set at login -> attacker subdomain reads cookie ->
       session hijacked (CWE-384)
"""
from flask import Flask, request, jsonify, make_response
from session import create_session, get_cookie_domain

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_alice", "role": "admin"},
    "dave":  {"password": "pw_dave",  "role": "user"},
}


# vuln-code-snippet start chain_cookie_scope_safe
@app.route("/login", methods=["POST"])
def login():
    """Authenticate the user and issue a session cookie scoped to the domain."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    token = create_session(username, user["role"])
    domain = get_cookie_domain()

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie(  # vuln-code-snippet safe-line chain_cookie_scope_safe
        "session", token, httponly=True, secure=True, domain=domain
    )
    return resp
# vuln-code-snippet end chain_cookie_scope_safe


@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session")
    return resp
