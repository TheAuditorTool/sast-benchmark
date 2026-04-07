from flask import jsonify
from module_b import app, USERS

_SAFE_FIELDS = {"username", "email"}

@app.route("/api/v1/users/<user_id>")
def get_user_v1(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify({k: v for k, v in user.items() if k in _SAFE_FIELDS})

@app.route("/api/v2/users/<user_id>")
def get_user_v2(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify({"username": user["username"], "email": user["email"]})
