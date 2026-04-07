import functools
import secrets
from flask import request, jsonify
from models import app, API_KEYS
from serializers import deserialize_key_create

_counter = [1]

def _require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if not request.headers.get("X-User-Id"):
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated

@app.route("/api-keys", methods=["POST"])
@_require_auth
def create_api_key():
    owner_id = request.headers.get("X-User-Id")
    data = request.get_json(force=True) or {}
    spec = deserialize_key_create(data)
    key_id = str(_counter[0])
    _counter[0] += 1
    API_KEYS[key_id] = {
        "owner_id": owner_id,
        "name": spec["name"],
        "scopes": spec["scopes"],
        "token": secrets.token_hex(16),
        "active": True,
    }
    return jsonify({"id": key_id, "scopes": spec["scopes"]}), 201

if __name__ == "__main__":
    app.run(port=5000)
