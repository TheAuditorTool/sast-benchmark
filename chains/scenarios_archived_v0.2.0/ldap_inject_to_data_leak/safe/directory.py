"""LDAP directory search -- SAFE variant.

GET /api/directory/search escapes all LDAP special characters in the
user-supplied 'uid' parameter before inserting it into the filter
string. Injected metacharacters are rendered as literal values and
cannot alter the search scope or return unintended entries.

Chain broken: escaped LDAP filter -> no injection -> admin DN not returned
"""
import re
from flask import Blueprint, request, jsonify

directory_bp = Blueprint("directory", __name__)

LDAP_DIRECTORY = {
    "uid=alice,dc=example,dc=com": {"uid": "alice", "cn": "Alice Smith"},
    "uid=bob,dc=example,dc=com": {"uid": "bob", "cn": "Bob Jones"},
    "uid=admin,dc=example,dc=com": {"uid": "admin", "cn": "Administrator"},
}

_LDAP_SPECIAL = re.compile(r'[\\*()\x00]')


def _escape_ldap(value):
    """Escape LDAP filter special characters."""
    return _LDAP_SPECIAL.sub(lambda m: "\\" + format(ord(m.group(0)), "02x"), value)


def _ldap_search(uid_filter):
    """Simulate an LDAP search returning matching DNs."""
    results = []
    for dn, attrs in LDAP_DIRECTORY.items():
        if uid_filter in ("*", attrs["uid"]):
            results.append(attrs["uid"])
    return results


# vuln-code-snippet start chain_ldap_leak_safe
@directory_bp.route("/api/directory/search")
def search_directory():
    """Search the directory for a user by uid."""
    uid = request.args.get("uid", "")
    safe_uid = _escape_ldap(uid)
    ldap_filter = "(&(objectClass=person)(uid=%s))" % safe_uid  # vuln-code-snippet safe-line chain_ldap_leak_safe
    matches = _ldap_search(safe_uid)
    return jsonify({"matches": matches, "filter": ldap_filter})
# vuln-code-snippet end chain_ldap_leak_safe
