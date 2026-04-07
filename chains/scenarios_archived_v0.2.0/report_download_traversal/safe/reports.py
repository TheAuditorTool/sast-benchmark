"""Report download blueprint -- SAFE variant.

GET /reports/download?name=<filename> strips all directory components from
the supplied name using os.path.basename before joining with REPORTS_DIR,
preventing traversal to files outside the reports directory.

Chain: broken -- basename strips directory components before open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, request, jsonify, Response
import config

reports_bp = Blueprint("reports", __name__)


@reports_bp.route("/reports/download")
def download_report():
    """Return the contents of a report file.

    FIXED: os.path.basename removes all directory components from the
    supplied name, so the resulting path can only reference files directly
    inside REPORTS_DIR.
    """
    filename = request.args.get("name", "")
    if not filename:
        return jsonify({"error": "name required"}), 400
    safe_name = os.path.basename(filename)
    if not safe_name:
        return jsonify({"error": "Invalid filename"}), 400
    report_path = os.path.join(config.REPORTS_DIR, safe_name)
# vuln-code-snippet start chain_report_traversal_safe
    with open(report_path, "r") as f:  # vuln-code-snippet safe-line chain_report_traversal_safe
        content = f.read()
# vuln-code-snippet end chain_report_traversal_safe
    return Response(content, mimetype="text/plain",
                    headers={"Content-Disposition": f"attachment; filename={safe_name}"})
