from flask import request, jsonify
from config import app, SESSIONS

# vuln-code-snippet start ChainScenario0175A
@app.after_request
def cors_headers(response):
    origin = request.headers.get("Origin", "")
    if origin:
        response.headers["Access-Control-Allow-Origin"] = origin  # vuln-code-snippet target-line ChainScenario0175A
        response.headers["Access-Control-Allow-Credentials"] = "true"
    return response
# vuln-code-snippet end ChainScenario0175A

@app.route("/api/token")
def get_token():
    session_id = request.cookies.get("session_id", "")
    session = SESSIONS.get(session_id)
    if session is None:
        return jsonify({"error": "Not authenticated"}), 401
    return jsonify({"api_token": session["api_token"]})
