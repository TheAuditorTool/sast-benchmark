import os
from flask import Blueprint, request, jsonify, Response
import config

reports_bp = Blueprint("reports", __name__)

@reports_bp.route("/reports/download")
def download_report():
    filename = request.args.get("name", "")
    if not filename:
        return jsonify({"error": "name required"}), 400
    safe_name = os.path.basename(filename)
    if not safe_name:
        return jsonify({"error": "Invalid filename"}), 400
    report_path = os.path.join(config.REPORTS_DIR, safe_name)
# vuln-code-snippet start ChainScenario0193B
    with open(report_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0193B
        content = f.read()
# vuln-code-snippet end ChainScenario0193B
    return Response(content, mimetype="text/plain",
                    headers={"Content-Disposition": f"attachment; filename={safe_name}"})
