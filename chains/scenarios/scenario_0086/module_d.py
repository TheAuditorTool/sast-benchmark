from flask import Blueprint, request, jsonify
from module_c import render_page

render_bp = Blueprint("render", __name__)

@render_bp.route("/render", methods=["POST"])
def render():
    content = request.json.get("template", "")
    output = render_page(content)
    return jsonify({"output": output})
