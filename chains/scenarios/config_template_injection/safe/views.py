"""Flask views for config_template_injection scenario -- SAFE variant.

POST /config passes the client-supplied value to engine.render_config,
which returns it as-is without template processing. No Jinja2 syntax
in the value is evaluated.

Chain broken: render_config returns literal -> no template expansion -> no RCE
"""
from flask import Blueprint, request, jsonify
from engine import render_config

config_bp = Blueprint("config", __name__)


# vuln-code-snippet start chain_config_template_safe
@config_bp.route("/config", methods=["POST"])
def apply_config():
    """Return the configuration value as plain text without template rendering."""
    value = request.json.get("value", "")
    context = request.json.get("context", {})
    rendered = render_config(value, context)  # vuln-code-snippet safe-line chain_config_template_safe
    return jsonify({"rendered": rendered})
# vuln-code-snippet end chain_config_template_safe
