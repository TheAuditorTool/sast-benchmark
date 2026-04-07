from flask import Blueprint, request, jsonify
from renderer import render_markdown

markdown_bp = Blueprint("markdown", __name__)

# vuln-code-snippet start ChainScenario0099B
@markdown_bp.route("/markdown", methods=["POST"])
def markdown():
    source = request.json.get("source", "")
    html = render_markdown(source)  # vuln-code-snippet target-line ChainScenario0099B
    return jsonify({"html": html})
# vuln-code-snippet end ChainScenario0099B
