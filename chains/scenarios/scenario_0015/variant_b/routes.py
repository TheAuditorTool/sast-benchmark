from flask import request, jsonify
from models import app, USERS, AUDIT_LOG
from auth import require_admin

# vuln-code-snippet start ChainScenario0015B
@app.route("/admin/users/<user_id>/reset-password", methods=["POST"])
@require_admin
def reset_user_password(user_id):
    target = USERS.get(user_id)
    if target is None:
        return jsonify({"error": "User not found"}), 404
    new_password = request.json.get("new_password", "temp1234")
    AUDIT_LOG.append({"action": "password_reset", "target": user_id, "by": request.current_user["username"]})
    return jsonify({"status": "reset", "user": user_id, "temp_password": new_password})  # vuln-code-snippet target-line ChainScenario0015B
# vuln-code-snippet end ChainScenario0015B

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

if __name__ == "__main__":
    app.run(port=5000)
