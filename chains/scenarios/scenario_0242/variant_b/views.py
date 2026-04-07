from flask import Blueprint, request, jsonify
from engine import render_template

message_bp = Blueprint("message", __name__)

# vuln-code-snippet start ChainScenario0242B
@message_bp.route("/message", methods=["POST"])
def message():
    template = request.json.get("template", "Hello, $username!")
    username = request.json.get("username", "guest")
    result = render_template(template, {"username": username})  # vuln-code-snippet target-line ChainScenario0242B
    return jsonify({"result": result})
# vuln-code-snippet end ChainScenario0242B
