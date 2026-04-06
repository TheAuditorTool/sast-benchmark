"""Report download blueprint -- VULNERABLE variant.

GET /reports/download?name=<filename> reads a report file using the
user-supplied filename joined directly with the reports directory.  A
filename like ../../etc/crontab or ../../app/config.py traverses out of
the reports directory and reads arbitrary files.

Chain: user-supplied report filename -> open() reads arbitrary file
Individual findings: missing path sanitization on report download (medium)
Chain finding: report filename traversal reads application secrets (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, request, jsonify, Response
import config

reports_bp = Blueprint("reports", __name__)


@reports_bp.route("/reports/download")
def download_report():
    """Return the contents of a report file.

    VULNERABLE: joins the user-supplied filename with REPORTS_DIR without
    stripping directory components or canonicalizing the path.
    """
    filename = request.args.get("name", "")
    if not filename:
        return jsonify({"error": "name required"}), 400
    report_path = os.path.join(config.REPORTS_DIR, filename)
# vuln-code-snippet start chain_report_traversal_vuln
    with open(report_path, "r") as f:  # vuln-code-snippet vuln-line chain_report_traversal_vuln
        content = f.read()
# vuln-code-snippet end chain_report_traversal_vuln
    return Response(content, mimetype="text/plain",
                    headers={"Content-Disposition": f"attachment; filename={filename}"})
