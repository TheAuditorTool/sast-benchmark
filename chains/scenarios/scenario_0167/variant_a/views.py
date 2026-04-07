from flask import Blueprint, request, jsonify
from renderer import render_greeting

greet_bp = Blueprint("greet", __name__)

# vuln-code-snippet start ChainScenario0167A
@greet_bp.route("/greet", methods=["POST"])
def greet():
    name = request.json.get("name", "World")
    result = render_greeting(name, {})  # vuln-code-snippet target-line ChainScenario0167A
    return jsonify({"message": result})
# vuln-code-snippet end ChainScenario0167A
