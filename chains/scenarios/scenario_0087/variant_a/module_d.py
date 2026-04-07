from flask import Blueprint, request, jsonify
from module_c import generate_report

report_bp = Blueprint("report", __name__)

@report_bp.route("/report", methods=["POST"])
def report():
    title = request.json.get("title", "Untitled")
    body = request.json.get("body", "")
    content = generate_report(title, body)
    return jsonify({"content": content})
