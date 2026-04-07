from flask import Flask, jsonify, session
from auth import login_required, get_current_user_id
from models import get_contacts_for_user, get_user_by_uuid, get_user_uuid

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

@app.route("/api/contacts")
@login_required
def list_contacts():
    uid = get_current_user_id()
    contacts = get_contacts_for_user(uid)
    safe_contacts = [
        {k: v for k, v in c.items() if k != "target_user_uuid"}
        for c in contacts
    ]
    return jsonify(safe_contacts)

@app.route("/api/users/<uuid>")
@login_required
def get_user_by_uuid_route(uuid):
    uid = get_current_user_id()
    own_uuid = get_user_uuid(uid)
    if own_uuid != uuid:
        return jsonify({"error": "Access denied"}), 403

# vuln-code-snippet start ChainScenario0120B
    user = get_user_by_uuid(uuid)  # vuln-code-snippet target-line ChainScenario0120B
# vuln-code-snippet end ChainScenario0120B
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(user)

if __name__ == "__main__":
    app.run(port=5000)
