from flask import request, jsonify
from module_a import app, MEMBERS
from module_c import clean_registration

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

@app.route("/register", methods=["POST"])
def register():
    data = request.get_json(force=True) or {}
    cleaned = clean_registration(data)
    username = cleaned["username"]
    if not username or not cleaned["email"]:
        return jsonify({"error": "username and email required"}), 400
    if username in MEMBERS:
        return jsonify({"error": "Username already exists"}), 409
    MEMBERS[username] = cleaned
    return jsonify({"status": "created", "member": cleaned}), 201

if __name__ == "__main__":
    app.run(port=5000)
