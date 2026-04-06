"""Registration routes -- SAFE variant.

POST /register now calls clean_registration() from the safe sanitize.py,
which forces role to 'member' and discards any user-supplied role value.

Chain: POST role=billing_admin -> clean_registration overrides to member -> chain broken
"""
from flask import request, jsonify
from models import app, MEMBERS
from sanitize import clean_registration


@app.route("/health")
def health():
    """Health check."""
    return jsonify({"status": "ok"})


# vuln-code-snippet start chain_role_param_safe
@app.route("/register", methods=["POST"])
def register():
    """Register a new organisation member. SAFE: role is always forced to member."""
    data = request.get_json(force=True) or {}
    cleaned = clean_registration(data)
    username = cleaned["username"]
    if not username or not cleaned["email"]:
        return jsonify({"error": "username and email required"}), 400
    if username in MEMBERS:
        return jsonify({"error": "Username already exists"}), 409
    MEMBERS[username] = cleaned  # vuln-code-snippet safe-line chain_role_param_safe
    return jsonify({"status": "created", "member": cleaned}), 201
# vuln-code-snippet end chain_role_param_safe


if __name__ == "__main__":
    app.run(port=5000)
