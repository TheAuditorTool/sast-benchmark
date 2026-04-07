"""Flask routes for the analytics reporting API -- VULNERABLE variant.

GET /api/events accepts a user_id query parameter and passes it
directly to the data layer.  Because the value is never validated
against the caller's session, any user can request another user's
event data by supplying a different user_id in the query string.

Chain: authenticated session + user_id param override -> another
  user's event stream
Individual findings: client-controlled filter not validated (medium)
Chain finding: bulk harvest of all users' behavioural analytics (high)
CWE-200: Exposure of Sensitive Information
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
    """Return events matching the supplied filter.

    VULNERABLE: user_id is taken from the query string without
    verifying it matches the authenticated user.  Supply any
    user_id to see that user's events.
    """
    event_type = request.args.get("type")
    limit = min(int(request.args.get("limit", 100)), 500)

# vuln-code-snippet start chain_api_filter_vuln
    target_user_id = int(request.args.get("user_id", get_current_user_id()))
    events = get_events_for_user(target_user_id, event_type, limit)  # vuln-code-snippet vuln-line chain_api_filter_vuln
# vuln-code-snippet end chain_api_filter_vuln

    return jsonify(events)


if __name__ == "__main__":
    app.run(port=5000)
