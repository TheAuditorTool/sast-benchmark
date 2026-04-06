"""Backup management blueprint -- SAFE variant.

GET /backups/download?path=<path> canonicalizes the path and verifies it
stays within the configured backups directory using realpath before opening.

Chain: broken -- path canonicalized and restricted to BACKUPS_DIR before open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, request, jsonify, Response
import config

backups_bp = Blueprint("backups", __name__)


@backups_bp.route("/backups/download")
def download_backup():
    """Stream a backup file to the caller.

    FIXED: realpath resolves all traversal sequences and symlinks; the
    resulting path must start with the backups directory before the file
    is opened.
    """
    backup_path = request.args.get("path", "")
    if not backup_path:
        return jsonify({"error": "path required"}), 400
    base = os.path.realpath(config.BACKUPS_DIR)
    full_path = os.path.realpath(os.path.join(config.BACKUPS_DIR, backup_path))
    if not full_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
# vuln-code-snippet start chain_backup_path_safe
    with open(full_path, "rb") as f:  # vuln-code-snippet safe-line chain_backup_path_safe
        data = f.read()
# vuln-code-snippet end chain_backup_path_safe
    filename = os.path.basename(full_path)
    return Response(data, mimetype="application/octet-stream",
                    headers={"Content-Disposition": f"attachment; filename={filename}"})
