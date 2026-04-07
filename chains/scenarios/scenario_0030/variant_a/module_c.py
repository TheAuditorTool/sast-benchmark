from flask import Blueprint, request, jsonify
from jinja2 import Environment

renderer_bp = Blueprint("renderer", __name__)
_jinja_env = Environment()

@renderer_bp.route("/render")
def render_template_endpoint():
    template_str = request.args.get("template", "Hello, world!")
    rendered = _jinja_env.from_string(template_str).render()
    return jsonify({"output": rendered})
