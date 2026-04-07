"""DevOps webhook routes -- SAFE variant.

POST /webhooks/provision now calls verify_webhook_signature() before
processing the payload. Requests without a valid HMAC signature are rejected.

Chain: unauthenticated POST -> signature verified -> 403 if invalid (chain broken)
"""
from flask import request, jsonify
from models import app, ADMIN_USERS
from auth import verify_webhook_signature


@app.route("/health")
def health():
    """Health check."""
    return jsonify({"status": "ok"})


# vuln-code-snippet start chain_webhook_admin_safe
@app.route("/webhooks/provision", methods=["POST"])
def provision_admin():
    """Provision an admin user from a webhook event. SAFE: HMAC signature required."""
    body = request.get_data()
    if not verify_webhook_signature(body):
        return jsonify({"error": "Invalid webhook signature"}), 403
    data = request.get_json(force=True) or {}
    username = data.get("username", "").strip()
    email = data.get("email", "").strip()
    if not username or not email:
        return jsonify({"error": "username and email required"}), 400
    ADMIN_USERS[username] = {"email": email, "role": "admin", "created_by": "webhook"}  # vuln-code-snippet safe-line chain_webhook_admin_safe
    return jsonify({"status": "provisioned", "username": username}), 201
# vuln-code-snippet end chain_webhook_admin_safe


if __name__ == "__main__":
    app.run(port=5000)
