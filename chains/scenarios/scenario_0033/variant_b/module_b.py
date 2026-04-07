import functools
from flask import request, jsonify
from module_a import app, PREFERENCES
from module_c import deserialize_preferences

def _require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-User-Id") not in PREFERENCES:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated

@app.route("/preferences", methods=["PUT"])
@_require_auth
def update_preferences():
    user_id = request.headers.get("X-User-Id")
    data = request.get_json(force=True) or {}
    updates = deserialize_preferences(data)
    PREFERENCES[user_id].update(updates)
    return jsonify({"status": "updated", "preferences": PREFERENCES[user_id]})

if __name__ == "__main__":
    app.run(port=5000)
