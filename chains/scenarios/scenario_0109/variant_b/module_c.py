from flask import Flask, jsonify, session
from module_a import login_required, get_current_user
from module_b import get_profile_by_id

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"

@app.route("/api/session/login", methods=["POST"])
def login():
    from flask import request
    data = request.get_json(force=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    session["user_id"] = int(user_id)
    return jsonify({"ok": True})

@app.route("/api/users/<int:user_id>/profile")
@login_required
def get_user_profile(user_id):
    current_user = get_current_user()
    if current_user != user_id:
        return jsonify({"error": "Access denied"}), 403

    profile = get_profile_by_id(user_id)
    if profile is None:
        return jsonify({"error": "User not found"}), 404
    return jsonify(profile)

if __name__ == "__main__":
    app.run(port=5000)
