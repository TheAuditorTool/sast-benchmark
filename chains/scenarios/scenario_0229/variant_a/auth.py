from flask import Blueprint, request, jsonify
import config

backup_bp = Blueprint("backup", __name__)

# vuln-code-snippet start ChainScenario0229A
@backup_bp.route("/backup", methods=["POST"])
def create_backup():
    label = request.json.get("label", "backup")
    backup_password = config.BACKUP_PASSWORD  # vuln-code-snippet target-line ChainScenario0229A
    return jsonify({
        "label": label,
        "dir": config.BACKUP_DIR,
        "encrypted": True,
        "password_length": len(backup_password),
    })
# vuln-code-snippet end ChainScenario0229A
