import functools
from flask import request, jsonify
from module_b import SESSION_STORE
from module_c import app

def _get_user_from_token():
    auth = request.headers.get("Authorization", "")
    if not auth.startswith("Bearer "):
        return None
    token = auth[len("Bearer "):]
    return SESSION_STORE.get(token)

def login_required(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user_id = _get_user_from_token()
        if user_id is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user_id = user_id
        return f(*args, **kwargs)
    return decorated

@app.route("/api/me")
@login_required
def me():
    return jsonify({"user_id": request.current_user_id})

@app.route("/api/admin/action", methods=["POST"])
@login_required
def admin_action():
    user_id = request.current_user_id
    if user_id != "admin":
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"status": "admin action performed"})

if __name__ == "__main__":
    app.run(port=5000)
