from flask import request, jsonify
from module_a import app, USERS
from module_b import deserialize_registration

@app.route("/register", methods=["POST"])
def register():
    data = request.get_json(force=True) or {}
    if not data.get("username") or not data.get("password"):
        return jsonify({"error": "username and password required"}), 400
    if data["username"] in USERS:
        return jsonify({"error": "Username taken"}), 409
    user = deserialize_registration(data)
    USERS[data["username"]] = user
    return jsonify({"status": "registered", "role": user.get("role", "user")}), 201

if __name__ == "__main__":
    app.run(port=5000)
