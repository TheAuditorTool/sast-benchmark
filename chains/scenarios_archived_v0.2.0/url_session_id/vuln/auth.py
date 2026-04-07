"""Authentication endpoints -- IDENTICAL between vuln/ and safe/ variants.

The login endpoint reads the sid query parameter (or cookie), creates/reuses
a session via session.py, then authenticates it.  Whether fixation succeeds
depends on session.py accepting attacker-supplied tokens from the URL.

Chain: ?sid=attacker_known_value -> POST /login -> session promoted ->
       attacker reuses ?sid= for authenticated requests (CWE-384)
"""
from flask import Flask, request, jsonify, make_response
from session import create_session, authenticate_session

app = Flask(__name__)

_USERS = {
    "alice": {"password": "secret1", "role": "admin"},
    "carol": {"password": "secret2", "role": "user"},
}


# vuln-code-snippet start chain_url_session_vuln
@app.route("/login", methods=["POST"])
def login():
    """Authenticate user; accept sid from URL or cookie for continuity."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    sid = request.cookies.get("sid") or request.args.get("sid", "")
    sid = create_session(sid if sid else None)
    sid = authenticate_session(sid, username, user["role"])  # vuln-code-snippet vuln-line chain_url_session_vuln

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("sid", sid, httponly=True)
    return resp
# vuln-code-snippet end chain_url_session_vuln


@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("sid")
    return resp
