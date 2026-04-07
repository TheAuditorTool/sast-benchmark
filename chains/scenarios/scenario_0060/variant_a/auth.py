from flask import Flask, request, jsonify, make_response
from session import create_session, promote_session, get_secure_flag

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_a", "role": "admin"},
    "eve":   {"password": "pw_e", "role": "user"},
}

@app.route("/init")
def init():
    token = create_session()
    resp = make_response(jsonify({"status": "session started"}))
    resp.set_cookie("session", token, httponly=True, secure=get_secure_flag())
    return resp

# vuln-code-snippet start ChainScenario0060A
@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    existing = request.cookies.get("session", "")
    token = promote_session(existing, username, user["role"])  # vuln-code-snippet target-line ChainScenario0060A

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("session", token, httponly=True, secure=get_secure_flag())
    return resp
# vuln-code-snippet end ChainScenario0060A

@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session")
    return resp
