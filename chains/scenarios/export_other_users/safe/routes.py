"""Flask routes for the data-export API -- SAFE variant.

The export endpoint ignores any user_id in the request body and
always exports only the authenticated caller's own data.

Chain: broken -- user_id sourced exclusively from session
CWE-200: Exposure of Sensitive Information (remediated)
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

    FIXED: user_id is always taken from the session, ignoring any
    body parameter.
    """
    uid = get_current_user_id()

# vuln-code-snippet start chain_export_idor_safe
    export = build_user_export(uid)  # vuln-code-snippet safe-line chain_export_idor_safe
# vuln-code-snippet end chain_export_idor_safe

    payload = json.dumps(export)
    return Response(payload, mimetype="application/json",
                    headers={"Content-Disposition": "attachment; filename=export.json"})


if __name__ == "__main__":
    app.run(port=5000)
