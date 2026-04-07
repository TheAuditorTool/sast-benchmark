"""Organisation invite routes.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. POST /orgs/<id>/invite carries a 'role' field in the body.
  2. deserialize_invite copies that role unchanged into the membership record.
  3. The new member holds whatever role the inviter specified, including 'owner'.

CWE-915: Mass assignment of org role enables privilege escalation.
"""
import functools
from flask import request, jsonify
from models import app, ORGS, MEMBERSHIPS
from serializers import deserialize_invite

_membership_counter = [1]


def _require_member(f):
    """Require X-User-Id to be an existing member of the org."""
    @functools.wraps(f)
    def decorated(org_id, *args, **kwargs):
        caller = request.headers.get("X-User-Id", "")
        if not any(m["org_id"] == org_id and m["user_id"] == caller
                   for m in MEMBERSHIPS.values()):
            if ORGS.get(org_id, {}).get("owner_id") != caller:
                return jsonify({"error": "Not a member"}), 403
        return f(org_id, *args, **kwargs)
    return decorated


@app.route("/orgs/<org_id>/invite", methods=["POST"])
@_require_member
def invite_member(org_id):
    """Invite a user to an organisation."""
    if org_id not in ORGS:
        return jsonify({"error": "Org not found"}), 404
    data = request.get_json(force=True) or {}
    membership = deserialize_invite(data)
    membership["org_id"] = org_id
    mid = str(_membership_counter[0])
    _membership_counter[0] += 1
    MEMBERSHIPS[mid] = membership
    return jsonify({"status": "invited", "membership": membership}), 201


if __name__ == "__main__":
    app.run(port=5000)
