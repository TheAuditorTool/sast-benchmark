"""Cloud API handler for service_account_key scenario -- SAFE variant.

GET /cloud-status authenticates using a service account key loaded from
an environment variable. No key material is present in source.

Chain broken: config.SERVICE_ACCOUNT_KEY from env -> no hardcoded key -> no unauthorized access
"""
import json
from flask import Blueprint, jsonify
import config

cloud_bp = Blueprint("cloud", __name__)


# vuln-code-snippet start chain_svc_account_safe
@cloud_bp.route("/cloud-status", methods=["GET"])
def cloud_status():
    """Return cloud connection status using an environment-sourced service account key."""
    key_info = config.SERVICE_ACCOUNT_KEY  # vuln-code-snippet safe-line chain_svc_account_safe
    return jsonify({
        "project": key_info.get("project_id"),
        "service_account": key_info.get("client_email"),
        "authenticated": True,
    })
# vuln-code-snippet end chain_svc_account_safe
