from flask import Blueprint, request, jsonify
from renderer import render_page

page_bp = Blueprint("page", __name__)

# vuln-code-snippet start ChainScenario0160B
@page_bp.route("/page", methods=["POST"])
def page():
    content = request.json.get("content", "")
    context = request.json.get("context", {})
    output = render_page(content, context)  # vuln-code-snippet target-line ChainScenario0160B
    return jsonify({"html": output})
# vuln-code-snippet end ChainScenario0160B
