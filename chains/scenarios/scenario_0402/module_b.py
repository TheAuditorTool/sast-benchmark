import functools
from flask import request, jsonify
from module_a import app, USERS
from module_c import validate_profile_update

def login_required(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-User-Id") not in USERS:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated

@app.route("/profile/<user_id>", methods=["PATCH"])
@login_required
def update_profile(user_id):
    caller_id = request.headers.get("X-User-Id")
    if caller_id != user_id and not USERS[caller_id].get("is_admin"):
        return jsonify({"error": "Forbidden"}), 403
    if user_id not in USERS:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    updates = validate_profile_update(data)
    USERS[user_id].update(updates)
    return jsonify({"status": "updated", "user": USERS[user_id]})

if __name__ == "__main__":
    app.run(port=5000)
