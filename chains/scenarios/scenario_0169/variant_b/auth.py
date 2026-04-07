from flask import Blueprint, request, jsonify
import config

ldap_bp = Blueprint("ldap", __name__)

# vuln-code-snippet start ChainScenario0169B
@ldap_bp.route("/ldap-auth", methods=["POST"])
def ldap_auth():
    username = request.json.get("username", "")
    bind_credentials = (config.LDAP_BIND_DN, config.LDAP_BIND_PASSWORD)  # vuln-code-snippet target-line ChainScenario0169B
    return jsonify({
        "bind_dn": bind_credentials[0],
        "host": config.LDAP_HOST,
        "lookup_user": username,
        "bound": True,
    })
# vuln-code-snippet end ChainScenario0169B
