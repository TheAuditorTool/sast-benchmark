"""LDAP authentication handler for ldap_bind_password scenario -- VULNERABLE variant.

POST /ldap-auth performs a simulated LDAP bind using the hardcoded
service account credentials to look up users. An attacker who reads
the source has the bind password and can connect to LDAP directly.

Chain: POST /ldap-auth -> config.LDAP_BIND_PASSWORD -> ldap bind -> directory access
"""
from flask import Blueprint, request, jsonify
import config

ldap_bp = Blueprint("ldap", __name__)


# vuln-code-snippet start chain_ldap_bind_vuln
@ldap_bp.route("/ldap-auth", methods=["POST"])
def ldap_auth():
    """Authenticate a user via LDAP using the hardcoded service account bind password."""
    username = request.json.get("username", "")
    bind_credentials = (config.LDAP_BIND_DN, config.LDAP_BIND_PASSWORD)  # vuln-code-snippet vuln-line chain_ldap_bind_vuln
    return jsonify({
        "bind_dn": bind_credentials[0],
        "host": config.LDAP_HOST,
        "lookup_user": username,
        "bound": True,
    })
# vuln-code-snippet end chain_ldap_bind_vuln
