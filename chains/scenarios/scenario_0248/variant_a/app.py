from flask import Flask, request, jsonify
from permissions import init_db, grant_permission, revoke_permission
from middleware import load_permissions, require_permission

app = Flask(__name__)

@app.before_request
def before():
    load_permissions()

@app.route("/admin/grant", methods=["POST"])
def grant():
    data = request.get_json(silent=True) or {}
    user_id = data.get("user_id")
    permission = data.get("permission")
    if not user_id or not permission:
        return jsonify({"error": "user_id and permission required"}), 400
    grant_permission(user_id, permission)
    return jsonify({"status": "granted"}), 200

@app.route("/admin/revoke", methods=["POST"])
def revoke():
    data = request.get_json(silent=True) or {}
    user_id = data.get("user_id")
    permission = data.get("permission")
    if not user_id or not permission:
        return jsonify({"error": "user_id and permission required"}), 400
    revoke_permission(user_id, permission)
    return jsonify({"status": "revoked"}), 200

@app.route("/resource/delete", methods=["DELETE"])
@require_permission("resource:delete")
def delete_resource():
    return jsonify({"status": "deleted"}), 200

@app.route("/resource/read", methods=["GET"])
@require_permission("resource:read")
def read_resource():
    return jsonify({"data": "sensitive content"}), 200

if __name__ == "__main__":
    init_db()
    app.run(port=5000)
