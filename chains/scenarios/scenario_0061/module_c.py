import os
from flask import Blueprint, request, jsonify, Response
import module_b

reports_bp = Blueprint("reports", __name__)

@reports_bp.route("/reports/download")
def download_report():
    filename = request.args.get("name", "")
    if not filename:
        return jsonify({"error": "name required"}), 400
    report_path = os.path.join(config.REPORTS_DIR, filename)
    with open(report_path, "r") as f:
        content = f.read()
    return Response(content, mimetype="text/plain",
                    headers={"Content-Disposition": f"attachment; filename={filename}"})
