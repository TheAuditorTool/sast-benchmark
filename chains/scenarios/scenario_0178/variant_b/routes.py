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
    uid = get_current_user_id()

# vuln-code-snippet start ChainScenario0178B
    export = build_user_export(uid)  # vuln-code-snippet target-line ChainScenario0178B
# vuln-code-snippet end ChainScenario0178B

    payload = json.dumps(export)
    return Response(payload, mimetype="application/json",
                    headers={"Content-Disposition": "attachment; filename=export.json"})

if __name__ == "__main__":
    app.run(port=5000)
