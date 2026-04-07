from flask import Blueprint, request, jsonify
from engine import render_config

config_bp = Blueprint("config", __name__)

# vuln-code-snippet start ChainScenario0232B
@config_bp.route("/config", methods=["POST"])
def apply_config():
    value = request.json.get("value", "")
    context = request.json.get("context", {})
    rendered = render_config(value, context)  # vuln-code-snippet target-line ChainScenario0232B
    return jsonify({"rendered": rendered})
# vuln-code-snippet end ChainScenario0232B
