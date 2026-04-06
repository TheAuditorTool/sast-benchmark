"""Flask routes for the data-export API -- VULNERABLE variant.

POST /api/export accepts a user_id in the request body and exports
all data for that user without verifying the caller owns the account.
An attacker can enumerate IDs to exfiltrate orders, addresses, and
support tickets for every user in the system.

Chain: authenticated session + arbitrary user_id body param -> full
  per-user data export
Individual findings: missing ownership check on export (medium)
Chain finding: bulk PII and order history exfiltration (high)
CWE-200: Exposure of Sensitive Information
"""
import json
from flask import Flask, jsonify, request, session, Response
from auth import login_required, get_current_user_id
from models import build_user_export

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"


@app.route("/api/session/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    session["user_id"] = int(user_id)
    return jsonify({"ok": True})


@app.route("/api/export", methods=["POST"])
@login_required
def export_data():
    """Generate and return a data export.

    VULNERABLE: user_id is taken from the request body without
    verifying it matches the session.  Any authenticated user can
    export another user's complete data.
    """
    body = request.get_json(force=True) or {}
    target_user_id = body.get("user_id", get_current_user_id())

# vuln-code-snippet start chain_export_idor_vuln
    export = build_user_export(int(target_user_id))  # vuln-code-snippet vuln-line chain_export_idor_vuln
# vuln-code-snippet end chain_export_idor_vuln

    payload = json.dumps(export)
    return Response(payload, mimetype="application/json",
                    headers={"Content-Disposition": "attachment; filename=export.json"})


if __name__ == "__main__":
    app.run(port=5000)
