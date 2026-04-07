from flask import Flask, jsonify, request, session
from auth import login_required, get_current_user_id
from models import get_events_for_user

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

@app.route("/api/events")
@login_required
def list_events():
    event_type = request.args.get("type")
    limit = min(int(request.args.get("limit", 100)), 500)
    uid = get_current_user_id()

# vuln-code-snippet start ChainScenario0010B
    events = get_events_for_user(uid, event_type, limit)  # vuln-code-snippet target-line ChainScenario0010B
# vuln-code-snippet end ChainScenario0010B

    return jsonify(events)

if __name__ == "__main__":
    app.run(port=5000)
