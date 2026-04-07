"""Template rendering endpoint -- SAFE variant.

GET /render accepts a user-supplied 'name' query parameter and inserts
it into a pre-compiled template using variable substitution only. The
user-supplied value is treated as data, never as template syntax, so
Jinja2 expression evaluation cannot be triggered.

Chain broken: pre-compiled template with data substitution -> no SSTI -> no file read
"""
from flask import Blueprint, request, jsonify
from jinja2 import Environment

renderer_bp = Blueprint("renderer", __name__)
_jinja_env = Environment()
_GREETING_TMPL = _jinja_env.from_string("Hello, {{ name }}!")


# vuln-code-snippet start chain_ssti_file_read_safe
@renderer_bp.route("/render")
def render_template_endpoint():
    """Render a greeting for the supplied name."""
    name = request.args.get("name", "world")
    rendered = _GREETING_TMPL.render(name=name)  # vuln-code-snippet safe-line chain_ssti_file_read_safe
    return jsonify({"output": rendered})
# vuln-code-snippet end chain_ssti_file_read_safe
