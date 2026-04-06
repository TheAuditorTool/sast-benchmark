"""Flask views for widget_render_ssti scenario -- VULNERABLE variant.

POST /widget accepts a JSON body representing a widget configuration.
The 'template' field is rendered as a Jinja2 template, enabling SSTI
if the attacker controls the widget config.

Chain: POST /widget -> renderer.render_widget(config) -> from_string -> RCE
"""
from flask import Blueprint, request, jsonify
from renderer import render_widget

widget_bp = Blueprint("widget", __name__)


# vuln-code-snippet start chain_widget_ssti_vuln
@widget_bp.route("/widget", methods=["POST"])
def widget():
    """Render a widget from its user-supplied configuration."""
    config = request.json or {}
    html = render_widget(config)  # vuln-code-snippet vuln-line chain_widget_ssti_vuln
    return jsonify({"html": html})
# vuln-code-snippet end chain_widget_ssti_vuln
