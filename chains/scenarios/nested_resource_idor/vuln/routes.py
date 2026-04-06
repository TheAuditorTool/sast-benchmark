"""Flask routes for the org team-admin API -- VULNERABLE variant.

GET /api/orgs/<org_id>/users/<user_id> returns full member details.
The handler joins on org_id and user_id so it only returns data when
the user actually belongs to that org -- BUT it never verifies that
the requesting user belongs to org_id.  An attacker can therefore
enumerate other orgs' members by varying org_id.

Chain: authenticated member of org A + guessed org_id for org B ->
  member details from org B
Individual findings: org membership of caller not verified (medium)
Chain finding: cross-org user directory enumeration (high)
CWE-862: Missing Authorization
"""
from flask import Flask, jsonify, session
from auth import login_required, get_current_user_id
from models import get_user_in_org, list_org_members, is_org_member

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"


@app.route("/api/session/login", methods=["POST"])
def login():
    from flask import request
    data = request.get_json(force=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    session["user_id"] = int(user_id)
    return jsonify({"ok": True})


@app.route("/api/orgs/<int:org_id>/users")
@login_required
def list_members(org_id):
    """Return org member list.

    VULNERABLE: does not verify the caller belongs to org_id.
    """
    return jsonify(list_org_members(org_id))


@app.route("/api/orgs/<int:org_id>/users/<int:user_id>")
@login_required
def get_member(org_id, user_id):
    """Return detailed member info.

    VULNERABLE: org_id is used to scope the query but the caller's
    own membership in that org is never checked.
    """
# vuln-code-snippet start chain_nested_resource_vuln
    member = get_user_in_org(org_id, user_id)  # vuln-code-snippet vuln-line chain_nested_resource_vuln
# vuln-code-snippet end chain_nested_resource_vuln
    if member is None:
        return jsonify({"error": "Member not found"}), 404
    return jsonify(member)


if __name__ == "__main__":
    app.run(port=5000)
