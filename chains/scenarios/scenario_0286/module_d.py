from flask import Blueprint, request, jsonify
from module_c import render_template

render_bp = Blueprint("render", __name__)

@render_bp.route("/render", methods=["POST"])
def render():
    template = request.json.get("template", "")
    context = request.json.get("context", {})
    output = render_template(template, context)
    return jsonify({"output": output})
