"""DevOps webhook routes -- VULNERABLE variant.

POST /webhooks/provision creates an admin user based on the webhook payload
without verifying the HMAC signature, allowing any caller to create an
admin account.

Chain: unauthenticated POST -> no signature check -> admin user inserted
"""
from flask import request, jsonify
from models import app, ADMIN_USERS
from auth import verify_webhook_signature


@app.route("/health")
def health():
    """Health check."""
    return jsonify({"status": "ok"})


# vuln-code-snippet start chain_webhook_admin_vuln
@app.route("/webhooks/provision", methods=["POST"])
def provision_admin():
    """Provision an admin user from a webhook event. VULNERABLE: signature not verified."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "").strip()
    email = data.get("email", "").strip()
    if not username or not email:
        return jsonify({"error": "username and email required"}), 400
    ADMIN_USERS[username] = {"email": email, "role": "admin", "created_by": "webhook"}  # vuln-code-snippet vuln-line chain_webhook_admin_vuln
    return jsonify({"status": "provisioned", "username": username}), 201
# vuln-code-snippet end chain_webhook_admin_vuln


if __name__ == "__main__":
    app.run(port=5000)
