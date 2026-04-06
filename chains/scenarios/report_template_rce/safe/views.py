"""Flask views for report_template_rce scenario -- SAFE variant.

POST /report passes the client-supplied title and body to
engine.generate_report, which uses plain string concatenation
with no eval. Injected code cannot execute.

Chain broken: generate_report uses string concat -> no eval -> no RCE
"""
from flask import Blueprint, request, jsonify
from engine import generate_report

report_bp = Blueprint("report", __name__)


# vuln-code-snippet start chain_report_template_safe
@report_bp.route("/report", methods=["POST"])
def report():
    """Generate a report using safe string concatenation."""
    title = request.json.get("title", "Untitled")
    body = request.json.get("body", "")
    content = generate_report(title, body)  # vuln-code-snippet safe-line chain_report_template_safe
    return jsonify({"content": content})
# vuln-code-snippet end chain_report_template_safe
