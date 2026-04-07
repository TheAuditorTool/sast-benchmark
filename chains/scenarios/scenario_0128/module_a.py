import functools
from flask import request, jsonify
from module_c import app
from module_b import JWT_SECRET

def _require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-JWT-Token") != JWT_SECRET:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated

@app.route("/admin/users")
@_require_auth
def list_users():
    return jsonify({"users": ["alice", "bob", "admin"]})

if __name__ == "__main__":
    app.run(port=5000)
