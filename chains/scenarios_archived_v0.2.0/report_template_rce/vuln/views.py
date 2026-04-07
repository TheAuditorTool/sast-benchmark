"""Flask views for report_template_rce scenario -- VULNERABLE variant.

POST /report accepts a JSON body with 'title' and 'body' fields.
The report engine evaluates a Python expression that concatenates the
user-supplied title, enabling RCE via injected code in the title.

Chain: POST /report -> engine.generate_report(title, body) -> eval -> RCE
"""
from flask import Blueprint, request, jsonify
from engine import generate_report

report_bp = Blueprint("report", __name__)


# vuln-code-snippet start chain_report_template_vuln
@report_bp.route("/report", methods=["POST"])
def report():
    """Generate a report from user-supplied title and body."""
    title = request.json.get("title", "Untitled")
    body = request.json.get("body", "")
    content = generate_report(title, body)  # vuln-code-snippet vuln-line chain_report_template_vuln
    return jsonify({"content": content})
# vuln-code-snippet end chain_report_template_vuln
