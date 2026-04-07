from flask import jsonify
from module_b import app, USERS

@app.route("/api/v1/users/<user_id>")
def get_user_v1(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(user)

@app.route("/api/v2/users/<user_id>")
def get_user_v2(user_id):
    user = USERS.get(user_id)
    if user is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify({"username": user["username"], "email": user["email"]})
