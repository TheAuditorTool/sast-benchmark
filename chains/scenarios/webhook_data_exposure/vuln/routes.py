"""Flask routes for the webhook delivery API -- VULNERABLE variant.

POST /api/webhooks/deliver triggers delivery of an event to all
registered webhook endpoints.  The handler builds the full event
payload (including all tenant-private fields) and delivers it to ALL
active webhook endpoints in the system -- not just the endpoints
belonging to the event's tenant.

Chain: any tenant can register a webhook -> delivery endpoint sends
  full payloads from all tenants' events to all endpoints
Individual findings: payload contains full event data (low) +
  no ACL on endpoint fan-out (medium)
Chain finding: cross-tenant data leakage via webhook delivery (high)
CWE-200: Exposure of Sensitive Information
"""
import json
import urllib.request
from flask import Flask, jsonify, request, session
from auth import login_required, get_current_user_id
from models import get_event, get_all_webhook_endpoints, get_tenant_for_user

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
    """Best-effort HTTP POST to a webhook endpoint."""
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
    """Dispatch an event to registered webhook endpoints.

    VULNERABLE: the full event record (including tenant-private payload)
    is delivered to ALL active webhook endpoints, not just those
    belonging to the event's tenant.
    """
    body = request.get_json(force=True) or {}
    event_id = body.get("event_id")
    if not event_id:
        return jsonify({"error": "event_id required"}), 400

# vuln-code-snippet start chain_webhook_exposure_vuln
    event = get_event(int(event_id))  # vuln-code-snippet vuln-line chain_webhook_exposure_vuln
# vuln-code-snippet end chain_webhook_exposure_vuln

    if event is None:
        return jsonify({"error": "Event not found"}), 404

    payload_bytes = json.dumps(event).encode("utf-8")
    endpoints = get_all_webhook_endpoints()
    for ep in endpoints:
        _deliver(ep["url"], ep["secret"], payload_bytes)

    return jsonify({"delivered": len(endpoints)})


if __name__ == "__main__":
    app.run(port=5000)
