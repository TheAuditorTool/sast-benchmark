from flask import Blueprint, request, jsonify
from engine import generate_report

report_bp = Blueprint("report", __name__)

# vuln-code-snippet start ChainScenario0087A
@report_bp.route("/report", methods=["POST"])
def report():
    title = request.json.get("title", "Untitled")
    body = request.json.get("body", "")
    content = generate_report(title, body)  # vuln-code-snippet target-line ChainScenario0087A
    return jsonify({"content": content})
# vuln-code-snippet end ChainScenario0087A
