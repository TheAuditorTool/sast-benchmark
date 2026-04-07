from flask import Blueprint, request, jsonify
import module_c

ldap_bp = Blueprint("ldap", __name__)

@ldap_bp.route("/ldap-auth", methods=["POST"])
def ldap_auth():
    username = request.json.get("username", "")
    bind_credentials = (config.LDAP_BIND_DN, config.LDAP_BIND_PASSWORD)
    return jsonify({
        "bind_dn": bind_credentials[0],
        "host": config.LDAP_HOST,
        "lookup_user": username,
        "bound": True,
    })
