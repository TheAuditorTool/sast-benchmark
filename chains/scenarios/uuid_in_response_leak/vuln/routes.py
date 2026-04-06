"""Flask routes for the contact directory API -- VULNERABLE variant.

Two-step chain:
  1. GET /api/contacts leaks target_user_uuid for every contact the
     caller can legitimately read.
  2. GET /api/users/<uuid> accepts any UUID and returns the full user
     record (including phone, ssn_last4, plan) with no ownership check.

An attacker harvests UUIDs from step 1 then pivots to step 2 to read
sensitive data for users they do not own.

Chain: uuid harvest from contacts -> unowned user lookup
Individual findings: sensitive field in list response (low) + missing
  ownership check on lookup (medium)
Chain finding: PII exfiltration of arbitrary users (high)
CWE-200: Exposure of Sensitive Information
"""
from flask import Flask, jsonify, session
from auth import login_required, get_current_user_id
from models import get_contacts_for_user, get_user_by_uuid

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

    VULNERABLE: the response includes target_user_uuid for each
    contact, giving the caller valid UUIDs to use against the
    lookup endpoint below.
    """
    uid = get_current_user_id()
    contacts = get_contacts_for_user(uid)
    return jsonify(contacts)


@app.route("/api/users/<uuid>")
@login_required
def get_user_by_uuid_route(uuid):
    """Return a user record by UUID.

    VULNERABLE: any authenticated user can look up any other user
    by UUID.  Combined with the leaked UUIDs above, this enables
    cross-user data access.
    """
# vuln-code-snippet start chain_uuid_leak_vuln
    user = get_user_by_uuid(uuid)  # vuln-code-snippet vuln-line chain_uuid_leak_vuln
# vuln-code-snippet end chain_uuid_leak_vuln
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(user)


if __name__ == "__main__":
    app.run(port=5000)
