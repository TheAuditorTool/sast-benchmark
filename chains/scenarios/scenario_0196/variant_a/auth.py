from flask import Blueprint, request, jsonify
from crypto import hash_password, verify_password

auth_bp = Blueprint("auth", __name__)

_DB: dict = {}

@auth_bp.route("/register", methods=["POST"])
def register():
    data = request.json or {}
    username = data.get("username", "")
    password = data.get("password", "")
    if not username or not password:
        return jsonify({"error": "username and password required"}), 400
    _DB[username] = hash_password(password)
    return jsonify({"status": "registered"})

# vuln-code-snippet start ChainScenario0196A
@auth_bp.route("/login", methods=["POST"])
def login():
    data = request.json or {}
    username = data.get("username", "")
    password = data.get("password", "")
    stored = _DB.get(username)
    if not stored or not verify_password(password, stored):
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "user": username})  # vuln-code-snippet target-line ChainScenario0196A
# vuln-code-snippet end ChainScenario0196A
