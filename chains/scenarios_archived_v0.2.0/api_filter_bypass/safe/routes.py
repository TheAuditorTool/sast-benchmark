"""Flask routes for the analytics reporting API -- SAFE variant.

The user_id filter parameter is ignored.  Event queries are always
scoped to the authenticated caller's id from the session.

Chain: broken -- filter parameter overridden by session identity
CWE-200: Exposure of Sensitive Information (remediated)
"""
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
    """Return events for the authenticated caller.

    FIXED: user_id is sourced exclusively from the session.  The
    query-string parameter is never used.
    """
    event_type = request.args.get("type")
    limit = min(int(request.args.get("limit", 100)), 500)
    uid = get_current_user_id()

# vuln-code-snippet start chain_api_filter_safe
    events = get_events_for_user(uid, event_type, limit)  # vuln-code-snippet safe-line chain_api_filter_safe
# vuln-code-snippet end chain_api_filter_safe

    return jsonify(events)


if __name__ == "__main__":
    app.run(port=5000)
