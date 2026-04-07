from flask import request, jsonify
from module_b import app, USERS, AUDIT_LOG
from module_a import require_admin

@app.route("/admin/users/<user_id>/reset-password", methods=["POST"])
@require_admin
def reset_user_password(user_id):
    target = USERS.get(user_id)
    if target is None:
        return jsonify({"error": "User not found"}), 404
    new_password = request.json.get("new_password", "temp1234")
    AUDIT_LOG.append({"action": "password_reset", "target": user_id, "by": request.current_user["username"]})
    return jsonify({"status": "reset", "user": user_id, "temp_password": new_password})

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

if __name__ == "__main__":
    app.run(port=5000)
