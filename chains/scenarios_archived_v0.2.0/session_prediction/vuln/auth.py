"""Authentication endpoints -- IDENTICAL between vuln/ and safe/ variants.

POST /login authenticates and issues a session token via session.py.
Whether the token is cryptographically random or predictable depends on
how session.py implements create_session().

Chain: sequential token counter -> attacker observes two tokens -> predicts
       token for target user -> replays predicted token (CWE-384)
"""
from flask import Flask, request, jsonify, make_response
from session import create_session

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_a", "role": "admin"},
    "bob":   {"password": "pw_b", "role": "user"},
    "carol": {"password": "pw_c", "role": "user"},
}


# vuln-code-snippet start chain_session_predict_vuln
@app.route("/login", methods=["POST"])
def login():
    """Authenticate the user and issue a session token."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    token = create_session(username, user["role"])  # vuln-code-snippet vuln-line chain_session_predict_vuln

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("session", token, httponly=True)
    return resp
# vuln-code-snippet end chain_session_predict_vuln


@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session")
    return resp
