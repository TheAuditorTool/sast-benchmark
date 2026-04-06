"""Flask views for widget_render_ssti scenario -- SAFE variant.

POST /widget passes the client config to renderer.render_widget.
The renderer selects a template from a server-side registry by the
config 'type' key; user-controlled values are context variables only.

Chain broken: template from server registry -> no user-controlled template source -> no SSTI
"""
from flask import Blueprint, request, jsonify
from renderer import render_widget

widget_bp = Blueprint("widget", __name__)


# vuln-code-snippet start chain_widget_ssti_safe
@widget_bp.route("/widget", methods=["POST"])
def widget():
    """Render a widget using a server-side template selected by config type."""
    config = request.json or {}
    html = render_widget(config)  # vuln-code-snippet safe-line chain_widget_ssti_safe
    return jsonify({"html": html})
# vuln-code-snippet end chain_widget_ssti_safe
