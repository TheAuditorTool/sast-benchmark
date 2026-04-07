from flask import Blueprint, request, jsonify
import module_c

backup_bp = Blueprint("backup", __name__)

@backup_bp.route("/backup", methods=["POST"])
def create_backup():
    label = request.json.get("label", "backup")
    backup_password = config.BACKUP_PASSWORD
    return jsonify({
        "label": label,
        "dir": config.BACKUP_DIR,
        "encrypted": True,
        "password_length": len(backup_password),
    })
