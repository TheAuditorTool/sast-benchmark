from flask import Blueprint, request, jsonify
from renderer import render_page

render_bp = Blueprint("render", __name__)

# vuln-code-snippet start ChainScenario0188B
@render_bp.route("/render", methods=["POST"])
def render():
    template_source = request.json.get("template", "")
    output = render_page(template_source)  # vuln-code-snippet target-line ChainScenario0188B
    return jsonify({"output": output})
# vuln-code-snippet end ChainScenario0188B
