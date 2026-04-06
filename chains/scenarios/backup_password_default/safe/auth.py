"""Backup handler for backup_password_default scenario -- SAFE variant.

POST /backup uses a backup password loaded from an environment variable.
No password is embedded in source.

Chain broken: config.BACKUP_PASSWORD from env -> no hardcoded password -> no decryption risk
"""
from flask import Blueprint, request, jsonify
import config

backup_bp = Blueprint("backup", __name__)


# vuln-code-snippet start chain_backup_pwd_safe
@backup_bp.route("/backup", methods=["POST"])
def create_backup():
    """Create a backup archive manifest using an environment-sourced backup password."""
    label = request.json.get("label", "backup")
    backup_password = config.BACKUP_PASSWORD  # vuln-code-snippet safe-line chain_backup_pwd_safe
    return jsonify({
        "label": label,
        "dir": config.BACKUP_DIR,
        "encrypted": True,
        "password_length": len(backup_password),
    })
# vuln-code-snippet end chain_backup_pwd_safe
