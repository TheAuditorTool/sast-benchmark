from flask import Flask, request, jsonify, make_response
from session import create_session, get_cookie_domain

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_alice", "role": "admin"},
    "dave":  {"password": "pw_dave",  "role": "user"},
}

# vuln-code-snippet start ChainScenario0122A
@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    token = create_session(username, user["role"])
    domain = get_cookie_domain()

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie(  # vuln-code-snippet target-line ChainScenario0122A
        "session", token, httponly=True, secure=True, domain=domain
    )
    return resp
# vuln-code-snippet end ChainScenario0122A

@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session")
    return resp
