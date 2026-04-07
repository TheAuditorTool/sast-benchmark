"""Flask routes for the webhook delivery API -- SAFE variant.

The delivery handler resolves the event's tenant and only delivers
to webhook endpoints registered by that same tenant.

Chain: broken -- delivery fan-out scoped to event's owning tenant
CWE-200: Exposure of Sensitive Information (remediated)
"""
import json
import urllib.request
from flask import Flask, jsonify, request, session
from auth import login_required, get_current_user_id
from models import get_event, get_webhook_endpoints_for_tenant, get_tenant_for_user

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

    FIXED: delivery targets only the endpoints registered by the
    tenant that owns the event.  Cross-tenant delivery is impossible.
    """
    body = request.get_json(force=True) or {}
    event_id = body.get("event_id")
    if not event_id:
        return jsonify({"error": "event_id required"}), 400

# vuln-code-snippet start chain_webhook_exposure_safe
    event = get_event(int(event_id))  # vuln-code-snippet safe-line chain_webhook_exposure_safe
# vuln-code-snippet end chain_webhook_exposure_safe

    if event is None:
        return jsonify({"error": "Event not found"}), 404

    event_tenant_id = event["tenant_id"]
    payload_bytes = json.dumps(event).encode("utf-8")
    endpoints = get_webhook_endpoints_for_tenant(event_tenant_id)
    for ep in endpoints:
        _deliver(ep["url"], ep["secret"], payload_bytes)

    return jsonify({"delivered": len(endpoints)})


if __name__ == "__main__":
    app.run(port=5000)
