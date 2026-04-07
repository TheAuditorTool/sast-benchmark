"""Template rendering endpoint -- VULNERABLE variant.

GET /render accepts a user-supplied 'template' query parameter and
renders it as a live Jinja2 template string. An attacker can submit
a payload like {{ config }} or {{ ''.__class__.__mro__[1].__subclasses__() }}
to read application configuration (including DB credentials from config.py)
or achieve arbitrary code execution.

Chain: SSTI reads config -> DB credentials in config.py are exfiltrated
Individual findings: SSTI (critical)
Chain finding: credential exfiltration via SSTI -> DB access (critical)
"""
from flask import Blueprint, request, jsonify
from jinja2 import Environment

renderer_bp = Blueprint("renderer", __name__)
_jinja_env = Environment()


# vuln-code-snippet start chain_ssti_file_read_vuln
@renderer_bp.route("/render")
def render_template_endpoint():
    """Render a user-supplied template string."""
    template_str = request.args.get("template", "Hello, world!")
    rendered = _jinja_env.from_string(template_str).render()  # vuln-code-snippet vuln-line chain_ssti_file_read_vuln
    return jsonify({"output": rendered})
# vuln-code-snippet end chain_ssti_file_read_vuln
