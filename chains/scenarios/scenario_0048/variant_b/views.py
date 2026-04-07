from flask import Blueprint, request, jsonify
from engine import render_template

render_bp = Blueprint("render", __name__)

# vuln-code-snippet start ChainScenario0048B
@render_bp.route("/render", methods=["POST"])
def render():
    template = request.json.get("template", "")
    context = request.json.get("context", {})
    output = render_template(template, context)  # vuln-code-snippet target-line ChainScenario0048B
    return jsonify({"output": output})
# vuln-code-snippet end ChainScenario0048B
