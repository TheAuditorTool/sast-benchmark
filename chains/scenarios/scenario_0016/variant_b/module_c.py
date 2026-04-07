from flask import Flask, jsonify, session
from module_a import login_required, get_current_user_id
from module_b import get_user_in_org, list_org_members, is_org_member

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
    return jsonify(list_org_members(org_id))

@app.route("/api/orgs/<int:org_id>/users/<int:user_id>")
@login_required
def get_member(org_id, user_id):
    member = get_user_in_org(org_id, user_id)
    if member is None:
        return jsonify({"error": "Member not found"}), 404
    return jsonify(member)

if __name__ == "__main__":
    app.run(port=5000)
