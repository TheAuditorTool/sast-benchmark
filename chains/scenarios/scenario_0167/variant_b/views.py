from flask import Blueprint, request, jsonify
from renderer import render_greeting

greet_bp = Blueprint("greet", __name__)

# vuln-code-snippet start ChainScenario0167B
@greet_bp.route("/greet", methods=["POST"])
def greet():
    name = request.json.get("name", "World")
    template_str = "Hello, " + name + "!"
    result = render_greeting(template_str, {})  # vuln-code-snippet target-line ChainScenario0167B
    return jsonify({"message": result})
# vuln-code-snippet end ChainScenario0167B
