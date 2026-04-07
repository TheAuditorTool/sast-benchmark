"""LDAP directory search -- VULNERABLE variant.

GET /api/directory/search accepts a user-supplied 'uid' parameter and
builds an LDAP filter via string formatting. An attacker can inject
LDAP metacharacters (e.g., *)(uid=admin)(&(uid=*)) to alter the filter
and cause the directory to return the admin DN. The returned username
is then passed to user_service to fetch sensitive attributes.

Chain: LDAP injection returns admin DN -> user_service leaks salary/SSN
Individual findings: LDAP injection (high)
Chain finding: sensitive attribute disclosure via LDAP injection (critical)
"""
from flask import Blueprint, request, jsonify

directory_bp = Blueprint("directory", __name__)

LDAP_DIRECTORY = {
    "uid=alice,dc=example,dc=com": {"uid": "alice", "cn": "Alice Smith"},
    "uid=bob,dc=example,dc=com": {"uid": "bob", "cn": "Bob Jones"},
    "uid=admin,dc=example,dc=com": {"uid": "admin", "cn": "Administrator"},
}


def _ldap_search(uid_filter):
    """Simulate an LDAP search returning matching DNs."""
    results = []
    for dn, attrs in LDAP_DIRECTORY.items():
        if uid_filter in ("*", attrs["uid"]):
            results.append(attrs["uid"])
    return results


# vuln-code-snippet start chain_ldap_leak_vuln
@directory_bp.route("/api/directory/search")
def search_directory():
    """Search the directory for a user by uid."""
    uid = request.args.get("uid", "")
    ldap_filter = "(&(objectClass=person)(uid=%s))" % uid  # vuln-code-snippet vuln-line chain_ldap_leak_vuln
    uid_part = uid
    matches = _ldap_search(uid_part)
    return jsonify({"matches": matches, "filter": ldap_filter})
# vuln-code-snippet end chain_ldap_leak_vuln
