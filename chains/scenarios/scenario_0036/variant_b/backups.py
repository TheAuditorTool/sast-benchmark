import os
from flask import Blueprint, request, jsonify, Response
import config

backups_bp = Blueprint("backups", __name__)

@backups_bp.route("/backups/download")
def download_backup():
    backup_path = request.args.get("path", "")
    if not backup_path:
        return jsonify({"error": "path required"}), 400
    full_path = os.path.join(config.BACKUPS_DIR, backup_path)
# vuln-code-snippet start ChainScenario0036B
    with open(full_path, "rb") as f:  # vuln-code-snippet target-line ChainScenario0036B
        data = f.read()
# vuln-code-snippet end ChainScenario0036B
    filename = os.path.basename(backup_path)
    return Response(data, mimetype="application/octet-stream",
                    headers={"Content-Disposition": f"attachment; filename={filename}"})
