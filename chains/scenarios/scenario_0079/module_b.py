import json
from flask import Blueprint, jsonify
import module_c

cloud_bp = Blueprint("cloud", __name__)

@cloud_bp.route("/cloud-status", methods=["GET"])
def cloud_status():
    key_info = config.SERVICE_ACCOUNT_KEY
    return jsonify({
        "project": key_info.get("project_id"),
        "service_account": key_info.get("client_email"),
        "authenticated": True,
    })
