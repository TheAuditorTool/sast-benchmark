import functools
from flask import request, jsonify
from config import app, SESSIONS
from debug import cors_headers, get_token  

def _require_api_token(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get("X-Api-Token", "")
        session = next(
            (s for s in SESSIONS.values() if s["api_token"] == token),
            None,
        )
        if session is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_session = session
        return f(*args, **kwargs)
    return decorated

@app.route("/api/admin/wipe", methods=["POST"])
@_require_api_token
def admin_wipe():
    if request.current_session.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    keys = [k for k, v in SESSIONS.items() if v["role"] != "admin"]
    for k in keys:
        del SESSIONS[k]
    return jsonify({"status": "wiped", "count": len(keys)})

if __name__ == "__main__":
    app.run(port=5000)
