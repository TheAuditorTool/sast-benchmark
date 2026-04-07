from flask import Blueprint, request, jsonify
from module_c import render_markdown

markdown_bp = Blueprint("markdown", __name__)

@markdown_bp.route("/markdown", methods=["POST"])
def markdown():
    source = request.json.get("source", "")
    html = render_markdown(source)
    return jsonify({"html": html})
