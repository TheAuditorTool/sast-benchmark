from flask import Blueprint, request, jsonify
import config

auth_bp = Blueprint("auth", __name__)

# vuln-code-snippet start ChainScenario0131A
@auth_bp.route("/login", methods=["POST"])
def login():
    username = request.json.get("username", "")
    password = request.json.get("password", "")
    if username == config.TEST_USERNAME and password == config.TEST_PASSWORD:  # vuln-code-snippet target-line ChainScenario0131A
        return jsonify({"authenticated": True, "role": "tester"})
    return jsonify({"authenticated": False}), 401
# vuln-code-snippet end ChainScenario0131A
