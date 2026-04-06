"""LDAP authentication handler for ldap_bind_password scenario -- SAFE variant.

POST /ldap-auth performs the LDAP bind using a password loaded from an
environment variable. No bind password is embedded in source.

Chain broken: config.LDAP_BIND_PASSWORD from env -> no hardcoded password -> no unauthorized bind
"""
from flask import Blueprint, request, jsonify
import config

ldap_bp = Blueprint("ldap", __name__)


# vuln-code-snippet start chain_ldap_bind_safe
@ldap_bp.route("/ldap-auth", methods=["POST"])
def ldap_auth():
    """Authenticate a user via LDAP using an environment-sourced bind password."""
    username = request.json.get("username", "")
    bind_credentials = (config.LDAP_BIND_DN, config.LDAP_BIND_PASSWORD)  # vuln-code-snippet safe-line chain_ldap_bind_safe
    return jsonify({
        "bind_dn": bind_credentials[0],
        "host": config.LDAP_HOST,
        "lookup_user": username,
        "bound": True,
    })
# vuln-code-snippet end chain_ldap_bind_safe
