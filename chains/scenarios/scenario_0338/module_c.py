from flask import request, jsonify
from module_b import app, SESSIONS

_ALLOWED_ORIGINS = {"https://app.example.com", "https://www.example.com"}

@app.after_request
def cors_headers(response):
    origin = request.headers.get("Origin", "")
    if origin in _ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
    return response

@app.route("/api/token")
def get_token():
    session_id = request.cookies.get("session_id", "")
    session = SESSIONS.get(session_id)
    if session is None:
        return jsonify({"error": "Not authenticated"}), 401
    return jsonify({"api_token": session["api_token"]})
