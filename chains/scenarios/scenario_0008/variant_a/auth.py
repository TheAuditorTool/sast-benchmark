from flask import Blueprint, request, jsonify
from crypto import generate_token

auth_bp = Blueprint("auth", __name__)

_SESSIONS: dict = {}

@auth_bp.route("/login", methods=["POST"])
def login():
    username = (request.json or {}).get("username", "guest")
    token = generate_token()
    _SESSIONS[token] = username
    return jsonify({"token": token})

# vuln-code-snippet start ChainScenario0008A
@auth_bp.route("/session")
def session():
    token = request.headers.get("X-Session-Token", "")
    username = _SESSIONS.get(token)
    if not username:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"user": username})  # vuln-code-snippet target-line ChainScenario0008A
# vuln-code-snippet end ChainScenario0008A
