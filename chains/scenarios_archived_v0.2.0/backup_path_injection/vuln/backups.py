"""Backup management blueprint -- VULNERABLE variant.

GET /backups/download?path=<path> serves a backup file at the user-supplied
path without restricting the path to the backups directory.  An attacker
can supply ../../etc/passwd or ../../app/config.py to read any file the
server process can access.

Chain: user-supplied backup path -> open() reads arbitrary file
Individual findings: unrestricted path in backup download (medium)
Chain finding: backup path injection reads database credentials or system files (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, request, jsonify, Response
import config

backups_bp = Blueprint("backups", __name__)


@backups_bp.route("/backups/download")
def download_backup():
    """Stream a backup file to the caller.

    VULNERABLE: joins user-supplied path with BACKUPS_DIR without
    canonicalization, so traversal sequences escape the backups directory.
    """
    backup_path = request.args.get("path", "")
    if not backup_path:
        return jsonify({"error": "path required"}), 400
    full_path = os.path.join(config.BACKUPS_DIR, backup_path)
# vuln-code-snippet start chain_backup_path_vuln
    with open(full_path, "rb") as f:  # vuln-code-snippet vuln-line chain_backup_path_vuln
        data = f.read()
# vuln-code-snippet end chain_backup_path_vuln
    filename = os.path.basename(backup_path)
    return Response(data, mimetype="application/octet-stream",
                    headers={"Content-Disposition": f"attachment; filename={filename}"})
