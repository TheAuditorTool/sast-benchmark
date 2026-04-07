import json
import urllib.request
from flask import Flask, jsonify, request, session
from module_a import login_required, get_current_user_id
from module_b import get_event, get_all_webhook_endpoints, get_tenant_for_user

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

def _deliver(url, secret, payload_bytes):
    try:
        req = urllib.request.Request(
            url,
            data=payload_bytes,
            headers={"Content-Type": "application/json",
                     "X-Webhook-Secret": secret},
            method="POST",
        )
        urllib.request.urlopen(req, timeout=5)
    except Exception:
        pass

@app.route("/api/webhooks/deliver", methods=["POST"])
@login_required
def deliver_webhook():
    body = request.get_json(force=True) or {}
    event_id = body.get("event_id")
    if not event_id:
        return jsonify({"error": "event_id required"}), 400

    event = get_event(int(event_id))

    if event is None:
        return jsonify({"error": "Event not found"}), 404

    payload_bytes = json.dumps(event).encode("utf-8")
    endpoints = get_all_webhook_endpoints()
    for ep in endpoints:
        _deliver(ep["url"], ep["secret"], payload_bytes)

    return jsonify({"delivered": len(endpoints)})

if __name__ == "__main__":
    app.run(port=5000)
