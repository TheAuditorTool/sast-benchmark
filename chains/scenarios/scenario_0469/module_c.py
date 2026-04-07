from flask import request, jsonify
from module_b import app, USERS
from module_a import validate_invite, consume_invite

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

@app.route("/signup", methods=["POST"])
def signup():
    data = request.get_json(force=True) or {}
    username = data.get("username", "").strip()
    email = data.get("email", "").strip()
    if not username or not email:
        return jsonify({"error": "username and email required"}), 400
    if username in USERS:
        return jsonify({"error": "Username taken"}), 409
    USERS[username] = {"email": email, "role": "user"}
    return jsonify({"status": "registered", "username": username}), 201

@app.route("/team/members", methods=["GET"])
def team_members():
    return jsonify({"members": list(USERS.keys())})

if __name__ == "__main__":
    app.run(port=5000)
