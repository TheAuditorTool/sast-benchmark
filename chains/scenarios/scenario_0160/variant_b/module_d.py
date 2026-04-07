from flask import Blueprint, request, jsonify
from module_c import render_page

page_bp = Blueprint("page", __name__)

@page_bp.route("/page", methods=["POST"])
def page():
    content = request.json.get("content", "")
    context = request.json.get("context", {})
    output = render_page(content, context)
    return jsonify({"html": output})
