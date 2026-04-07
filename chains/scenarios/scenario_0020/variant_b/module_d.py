from flask import Blueprint, request, jsonify
from module_c import render_widget

widget_bp = Blueprint("widget", __name__)

@widget_bp.route("/widget", methods=["POST"])
def widget():
    config = request.json or {}
    html = render_widget(config)
    return jsonify({"html": html})
