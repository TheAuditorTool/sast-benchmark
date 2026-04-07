from flask import Flask, request, jsonify, make_response
from session import create_session

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_a", "role": "admin"},
    "bob":   {"password": "pw_b", "role": "user"},
    "carol": {"password": "pw_c", "role": "user"},
}

# vuln-code-snippet start ChainScenario0103B
@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    token = create_session(username, user["role"])  # vuln-code-snippet target-line ChainScenario0103B

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("session", token, httponly=True)
    return resp
# vuln-code-snippet end ChainScenario0103B

@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session")
    return resp
