from flask import Flask, request, jsonify, make_response
from session import get_or_create_session, promote_session

app = Flask(__name__)

_USERS = {
    "alice": {"password": "hunter2", "role": "admin"},
    "bob":   {"password": "pass1234", "role": "user"},
}

# vuln-code-snippet start ChainScenario0195B
@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    existing_token = request.cookies.get("session_token", "")
    _, _ = get_or_create_session(existing_token)

    new_token = promote_session(existing_token, username, user["role"])  # vuln-code-snippet target-line ChainScenario0195B

    resp = make_response(jsonify({"status": "logged in", "user": username}))
    resp.set_cookie("session_token", new_token, httponly=True)
    return resp
# vuln-code-snippet end ChainScenario0195B

@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session_token")
    return resp
