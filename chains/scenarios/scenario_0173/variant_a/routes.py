from flask import request, jsonify
from models import app, ADMIN_USERS
from auth import verify_webhook_signature

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

# vuln-code-snippet start ChainScenario0173A
@app.route("/webhooks/provision", methods=["POST"])
def provision_admin():
    body = request.get_data()
    if not verify_webhook_signature(body):
        return jsonify({"error": "Invalid webhook signature"}), 403
    data = request.get_json(force=True) or {}
    username = data.get("username", "").strip()
    email = data.get("email", "").strip()
    if not username or not email:
        return jsonify({"error": "username and email required"}), 400
    ADMIN_USERS[username] = {"email": email, "role": "admin", "created_by": "webhook"}  # vuln-code-snippet target-line ChainScenario0173A
    return jsonify({"status": "provisioned", "username": username}), 201
# vuln-code-snippet end ChainScenario0173A

if __name__ == "__main__":
    app.run(port=5000)
