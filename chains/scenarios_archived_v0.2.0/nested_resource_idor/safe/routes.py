"""Flask routes for the org team-admin API -- SAFE variant.

Both endpoints verify that the requesting user is a member of org_id
before returning any org data.

Chain: broken -- caller membership in org enforced before retrieval
CWE-862: Missing Authorization (remediated)
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

    FIXED: caller must be a member of org_id.
    """
    uid = get_current_user_id()
    if not is_org_member(org_id, uid):
        return jsonify({"error": "Access denied"}), 403
    return jsonify(list_org_members(org_id))


@app.route("/api/orgs/<int:org_id>/users/<int:user_id>")
@login_required
def get_member(org_id, user_id):
    """Return detailed member info.

    FIXED: caller membership in org_id is verified before any
    member data is retrieved.
    """
    uid = get_current_user_id()
    if not is_org_member(org_id, uid):
        return jsonify({"error": "Access denied"}), 403

# vuln-code-snippet start chain_nested_resource_safe
    member = get_user_in_org(org_id, user_id)  # vuln-code-snippet safe-line chain_nested_resource_safe
# vuln-code-snippet end chain_nested_resource_safe
    if member is None:
        return jsonify({"error": "Member not found"}), 404
    return jsonify(member)


if __name__ == "__main__":
    app.run(port=5000)
