from flask import Blueprint, request, jsonify
from renderer import render_widget

widget_bp = Blueprint("widget", __name__)

# vuln-code-snippet start ChainScenario0020B
@widget_bp.route("/widget", methods=["POST"])
def widget():
    config = request.json or {}
    html = render_widget(config)  # vuln-code-snippet target-line ChainScenario0020B
    return jsonify({"html": html})
# vuln-code-snippet end ChainScenario0020B
