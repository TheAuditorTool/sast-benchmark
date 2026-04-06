"""Backup handler for backup_password_default scenario -- VULNERABLE variant.

POST /backup creates a simulated backup manifest using the hardcoded
password. Because the password is a known constant, any attacker can
decrypt archives produced by this endpoint.

Chain: POST /backup -> config.BACKUP_PASSWORD -> archive encrypted with known key
"""
from flask import Blueprint, request, jsonify
import config

backup_bp = Blueprint("backup", __name__)


# vuln-code-snippet start chain_backup_pwd_vuln
@backup_bp.route("/backup", methods=["POST"])
def create_backup():
    """Create a backup archive manifest using the hardcoded backup password."""
    label = request.json.get("label", "backup")
    backup_password = config.BACKUP_PASSWORD  # vuln-code-snippet vuln-line chain_backup_pwd_vuln
    return jsonify({
        "label": label,
        "dir": config.BACKUP_DIR,
        "encrypted": True,
        "password_length": len(backup_password),
    })
# vuln-code-snippet end chain_backup_pwd_vuln
