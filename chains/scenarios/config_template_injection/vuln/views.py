"""Flask views for config_template_injection scenario -- VULNERABLE variant.

POST /config accepts a JSON body with a 'value' field representing a
configuration string. The value is rendered as a Jinja2 template,
allowing an attacker to inject expressions and achieve RCE.

Chain: POST /config -> engine.render_config(value) -> from_string -> RCE
"""
from flask import Blueprint, request, jsonify
from engine import render_config

config_bp = Blueprint("config", __name__)


# vuln-code-snippet start chain_config_template_vuln
@config_bp.route("/config", methods=["POST"])
def apply_config():
    """Render a configuration value as a Jinja2 template."""
    value = request.json.get("value", "")
    context = request.json.get("context", {})
    rendered = render_config(value, context)  # vuln-code-snippet vuln-line chain_config_template_vuln
    return jsonify({"rendered": rendered})
# vuln-code-snippet end chain_config_template_vuln
