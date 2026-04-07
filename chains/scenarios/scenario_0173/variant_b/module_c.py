from flask import request, jsonify
from module_b import app, ADMIN_USERS
from module_a import verify_webhook_signature

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

@app.route("/webhooks/provision", methods=["POST"])
def provision_admin():
    data = request.get_json(force=True) or {}
    username = data.get("username", "").strip()
    email = data.get("email", "").strip()
    if not username or not email:
        return jsonify({"error": "username and email required"}), 400
    ADMIN_USERS[username] = {"email": email, "role": "admin", "created_by": "webhook"}
    return jsonify({"status": "provisioned", "username": username}), 201

if __name__ == "__main__":
    app.run(port=5000)
