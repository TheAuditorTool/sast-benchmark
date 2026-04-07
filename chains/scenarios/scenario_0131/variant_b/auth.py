from flask import Blueprint, request, jsonify

auth_bp = Blueprint("auth", __name__)

_USER_STORE = {}

# vuln-code-snippet start ChainScenario0131B
@auth_bp.route("/login", methods=["POST"])
def login():
    username = request.json.get("username", "")
    password = request.json.get("password", "")
    stored = _USER_STORE.get(username)  # vuln-code-snippet target-line ChainScenario0131B
    if stored and stored == password:
        return jsonify({"authenticated": True, "role": "user"})
    return jsonify({"authenticated": False}), 401
# vuln-code-snippet end ChainScenario0131B
