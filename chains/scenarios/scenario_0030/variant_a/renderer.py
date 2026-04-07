from flask import Blueprint, request, jsonify
from jinja2 import Environment

renderer_bp = Blueprint("renderer", __name__)
_jinja_env = Environment()

# vuln-code-snippet start ChainScenario0030A
@renderer_bp.route("/render")
def render_template_endpoint():
    template_str = request.args.get("template", "Hello, world!")
    rendered = _jinja_env.from_string(template_str).render()  # vuln-code-snippet target-line ChainScenario0030A
    return jsonify({"output": rendered})
# vuln-code-snippet end ChainScenario0030A
