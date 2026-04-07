from flask import request, jsonify
from models import app, MEMBERS
from sanitize import clean_registration

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

# vuln-code-snippet start ChainScenario0101A
@app.route("/register", methods=["POST"])
def register():
    data = request.get_json(force=True) or {}
    cleaned = clean_registration(data)
    username = cleaned["username"]
    if not username or not cleaned["email"]:
        return jsonify({"error": "username and email required"}), 400
    if username in MEMBERS:
        return jsonify({"error": "Username already exists"}), 409
    MEMBERS[username] = cleaned  # vuln-code-snippet target-line ChainScenario0101A
    return jsonify({"status": "created", "member": cleaned}), 201
# vuln-code-snippet end ChainScenario0101A

if __name__ == "__main__":
    app.run(port=5000)
