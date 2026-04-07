"""Cloud API handler for service_account_key scenario -- VULNERABLE variant.

GET /cloud-status authenticates to a cloud API using the hardcoded
service account key. An attacker with source access has the full key
and can make authenticated cloud API calls independently.

Chain: GET /cloud-status -> config.SERVICE_ACCOUNT_KEY -> cloud API -> resource access
"""
import json
from flask import Blueprint, jsonify
import config

cloud_bp = Blueprint("cloud", __name__)


# vuln-code-snippet start chain_svc_account_vuln
@cloud_bp.route("/cloud-status", methods=["GET"])
def cloud_status():
    """Return cloud connection status using the hardcoded service account key."""
    key_info = config.SERVICE_ACCOUNT_KEY  # vuln-code-snippet vuln-line chain_svc_account_vuln
    return jsonify({
        "project": key_info.get("project_id"),
        "service_account": key_info.get("client_email"),
        "authenticated": True,
    })
# vuln-code-snippet end chain_svc_account_vuln
