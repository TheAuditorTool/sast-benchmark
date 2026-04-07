from flask import Flask, request, jsonify, make_response
from module_c import create_session, authenticate_session

app = Flask(__name__)

_USERS = {
    "alice": {"password": "secret1", "role": "admin"},
    "carol": {"password": "secret2", "role": "user"},
}

@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    user = _USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401

    sid = request.cookies.get("sid") or request.args.get("sid", "")
    sid = create_session(sid if sid else None)
    sid = authenticate_session(sid, username, user["role"])

    resp = make_response(jsonify({"status": "logged in"}))
    resp.set_cookie("sid", sid, httponly=True)
    return resp

@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("sid")
    return resp
