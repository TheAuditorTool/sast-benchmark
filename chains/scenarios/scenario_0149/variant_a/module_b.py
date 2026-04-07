from flask import Flask, request, jsonify, make_response
from module_c import get_or_create_session, promote_session

app = Flask(__name__)

_USERS = {
    "alice": {"password": "pw_a", "role": "admin"},
    "bob":   {"password": "pw_b", "role": "user"},
}

@app.route("/")
def index():
    token, _ = get_or_create_session()
    resp = make_response(jsonify({"status": "welcome"}))
    resp.set_cookie("session", token, httponly=True)
    return resp

@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    existing_token = request.cookies.get("session", "")
    token = promote_session(existing_token, username, user["role"])

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("session", token, httponly=True)
    return resp

@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("session")
    return resp
