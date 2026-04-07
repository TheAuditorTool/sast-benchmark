from flask import Flask, request, jsonify, make_response
from session_store import issue_remember_token, restore_session_from_remember

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_alice", "role": "admin"},
    "bob":   {"password": "pw_bob",   "role": "user"},
}

@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    remember = data.get("remember", False)

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    resp = make_response(jsonify({"status": "logged in"}))
    if remember:
        rtoken = issue_remember_token(username, user["role"])
        resp.set_cookie("remember_token", rtoken, httponly=True, max_age=30 * 86400)
    return resp

# vuln-code-snippet start ChainScenario0035B
@app.route("/remember", methods=["POST"])
def remember():
    rtoken = request.cookies.get("remember_token", "")
    session_token, new_rtoken = restore_session_from_remember(rtoken)  # vuln-code-snippet target-line ChainScenario0035B
    if session_token is None:
        return jsonify({"error": "Invalid or expired remember token"}), 401

    resp = make_response(jsonify({"status": "session restored"}))
    resp.set_cookie("session_token", session_token, httponly=True)
    resp.set_cookie("remember_token", new_rtoken, httponly=True, max_age=30 * 86400)
    return resp
# vuln-code-snippet end ChainScenario0035B

@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session_token")
    resp.delete_cookie("remember_token")
    return resp
