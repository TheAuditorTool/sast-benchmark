from flask import Blueprint, request, jsonify
from module_c import render_config

config_bp = Blueprint("config", __name__)

@config_bp.route("/config", methods=["POST"])
def apply_config():
    value = request.json.get("value", "")
    context = request.json.get("context", {})
    rendered = render_config(value, context)
    return jsonify({"rendered": rendered})
