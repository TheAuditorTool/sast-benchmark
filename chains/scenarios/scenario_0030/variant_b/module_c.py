from flask import Blueprint, request, jsonify
from jinja2 import Environment

renderer_bp = Blueprint("renderer", __name__)
_jinja_env = Environment()
_GREETING_TMPL = _jinja_env.from_string("Hello, {{ name }}!")

@renderer_bp.route("/render")
def render_template_endpoint():
    name = request.args.get("name", "world")
    rendered = _GREETING_TMPL.render(name=name)
    return jsonify({"output": rendered})
