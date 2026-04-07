import functools
from flask import request, jsonify
from module_a import app, ORGS, MEMBERSHIPS
from module_c import deserialize_invite

_membership_counter = [1]

def _require_member(f):
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
