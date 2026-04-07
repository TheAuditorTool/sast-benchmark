import os
from flask import Blueprint, request, jsonify, Response
import module_c

backups_bp = Blueprint("backups", __name__)

@backups_bp.route("/backups/download")
def download_backup():
    backup_path = request.args.get("path", "")
    if not backup_path:
        return jsonify({"error": "path required"}), 400
    full_path = os.path.join(config.BACKUPS_DIR, backup_path)
    with open(full_path, "rb") as f:
        data = f.read()
    filename = os.path.basename(backup_path)
    return Response(data, mimetype="application/octet-stream",
                    headers={"Content-Disposition": f"attachment; filename={filename}"})
