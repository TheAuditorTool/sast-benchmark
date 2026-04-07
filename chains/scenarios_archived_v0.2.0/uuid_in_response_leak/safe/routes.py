"""Flask routes for the contact directory API -- SAFE variant.

The contacts list no longer exposes target_user_uuid (step 1 of
the chain is broken).  The user lookup endpoint additionally requires
that the UUID matches the caller's own UUID.

Chain: broken -- UUIDs stripped from list response; lookup is gated
CWE-200: Exposure of Sensitive Information (remediated)
"""
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
    """Return the caller's contact list.

    FIXED: target_user_uuid is stripped before returning so callers
    cannot harvest UUIDs for unowned accounts.
    """
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
    """Return a user record by UUID.

    FIXED: the endpoint verifies that the requested UUID belongs to
    the caller before returning any data.
    """
    uid = get_current_user_id()
    own_uuid = get_user_uuid(uid)
    if own_uuid != uuid:
        return jsonify({"error": "Access denied"}), 403

# vuln-code-snippet start chain_uuid_leak_safe
    user = get_user_by_uuid(uuid)  # vuln-code-snippet safe-line chain_uuid_leak_safe
# vuln-code-snippet end chain_uuid_leak_safe
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(user)


if __name__ == "__main__":
    app.run(port=5000)
