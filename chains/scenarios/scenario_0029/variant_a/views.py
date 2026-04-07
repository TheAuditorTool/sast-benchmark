from flask import request, jsonify
from models import app, USERS
from auth import validate_invite, consume_invite

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

# vuln-code-snippet start ChainScenario0029A
@app.route("/signup", methods=["POST"])
def signup():
    data = request.get_json(force=True) or {}
    username = data.get("username", "").strip()
    email = data.get("email", "").strip()
    invite_token = data.get("invite_token", "").strip()
    if not username or not email:
        return jsonify({"error": "username and email required"}), 400
    if not validate_invite(invite_token):
        return jsonify({"error": "Valid invite token required"}), 403
    if username in USERS:
        return jsonify({"error": "Username taken"}), 409
    consume_invite(invite_token)
    USERS[username] = {"email": email, "role": "user"}  # vuln-code-snippet target-line ChainScenario0029A
    return jsonify({"status": "registered", "username": username}), 201
# vuln-code-snippet end ChainScenario0029A

@app.route("/team/members", methods=["GET"])
def team_members():
    return jsonify({"members": list(USERS.keys())})

if __name__ == "__main__":
    app.run(port=5000)
