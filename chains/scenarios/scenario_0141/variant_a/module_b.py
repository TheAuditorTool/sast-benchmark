from flask import Blueprint, request, jsonify

directory_bp = Blueprint("directory", __name__)

LDAP_DIRECTORY = {
    "uid=alice,dc=example,dc=com": {"uid": "alice", "cn": "Alice Smith"},
    "uid=bob,dc=example,dc=com": {"uid": "bob", "cn": "Bob Jones"},
    "uid=admin,dc=example,dc=com": {"uid": "admin", "cn": "Administrator"},
}

def _ldap_search(uid_filter):
    results = []
    for dn, attrs in LDAP_DIRECTORY.items():
        if uid_filter in ("*", attrs["uid"]):
            results.append(attrs["uid"])
    return results

@directory_bp.route("/api/directory/search")
def search_directory():
    uid = request.args.get("uid", "")
    ldap_filter = "(&(objectClass=person)(uid=%s))" % uid
    uid_part = uid
    matches = _ldap_search(uid_part)
    return jsonify({"matches": matches, "filter": ldap_filter})
