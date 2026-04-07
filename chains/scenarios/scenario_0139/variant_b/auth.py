import json
from flask import Blueprint, jsonify
import config

cloud_bp = Blueprint("cloud", __name__)

# vuln-code-snippet start ChainScenario0139B
@cloud_bp.route("/cloud-status", methods=["GET"])
def cloud_status():
    key_info = config.SERVICE_ACCOUNT_KEY  # vuln-code-snippet target-line ChainScenario0139B
    return jsonify({
        "project": key_info.get("project_id"),
        "service_account": key_info.get("client_email"),
        "authenticated": True,
    })
# vuln-code-snippet end ChainScenario0139B
