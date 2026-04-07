import functools
from flask import request, jsonify
from module_a import app, USERS
from module_c import deserialize_profile_update

def _require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-User-Id") not in USERS:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated

@app.route("/profile", methods=["PATCH"])
@_require_auth
def update_profile():
    user_id = request.headers.get("X-User-Id")
    data = request.get_json(force=True) or {}
    existing_settings = dict(USERS[user_id].get("settings", {}))
    updates = deserialize_profile_update(data, existing_settings)
    USERS[user_id].update(updates)
    return jsonify({"status": "updated", "user": USERS[user_id]})

if __name__ == "__main__":
    app.run(port=5000)
