"""Authentication endpoints -- IDENTICAL between vuln/ and safe/ variants.

GET / calls get_or_create_session() to ensure every visitor has a session.
POST /login promotes the current session to authenticated.
Whether the token is regenerated at login (preventing overflow fixation) depends
on session.py.

Chain: cookie jar flooded -> legitimate cookie evicted -> new anonymous session
       assigned (attacker can predict/plant it) -> /login promotes planted token
       -> attacker reuses it (CWE-384)
"""
from flask import Flask, request, jsonify, make_response
from session import get_or_create_session, promote_session

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_a", "role": "admin"},
    "bob":   {"password": "pw_b", "role": "user"},
}


@app.route("/")
def index():
    """Landing page: ensure the visitor has a session cookie."""
    token, _ = get_or_create_session()
    resp = make_response(jsonify({"status": "welcome"}))
    resp.set_cookie("session", token, httponly=True)
    return resp


# vuln-code-snippet start chain_cookie_overflow_safe
@app.route("/login", methods=["POST"])
def login():
    """Authenticate and promote the current session to authenticated."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    existing_token = request.cookies.get("session", "")
    token = promote_session(existing_token, username, user["role"])  # vuln-code-snippet safe-line chain_cookie_overflow_safe

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("session", token, httponly=True)
    return resp
# vuln-code-snippet end chain_cookie_overflow_safe


@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session")
    return resp
