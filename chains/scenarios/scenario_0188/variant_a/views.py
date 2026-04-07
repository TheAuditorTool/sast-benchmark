from flask import Blueprint, request, jsonify
from renderer import render_page

render_bp = Blueprint("render", __name__)

# vuln-code-snippet start ChainScenario0188A
@render_bp.route("/render", methods=["POST"])
def render():
    content = request.json.get("template", "")
    output = render_page(content)  # vuln-code-snippet target-line ChainScenario0188A
    return jsonify({"output": output})
# vuln-code-snippet end ChainScenario0188A
