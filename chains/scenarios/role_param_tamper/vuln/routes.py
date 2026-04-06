"""Registration routes -- VULNERABLE variant.

POST /register passes the request body through clean_registration() which
preserves the user-supplied role field, allowing self-assignment of elevated
roles such as billing_admin or owner.

Chain: POST role=billing_admin -> clean_registration preserves it -> privileged member created
"""
from flask import request, jsonify
from models import app, MEMBERS
from sanitize import clean_registration


@app.route("/health")
def health():
    """Health check."""
    return jsonify({"status": "ok"})


# vuln-code-snippet start chain_role_param_vuln
@app.route("/register", methods=["POST"])
def register():
    """Register a new organisation member. VULNERABLE: role accepted from request body."""
    data = request.get_json(force=True) or {}
    cleaned = clean_registration(data)
    username = cleaned["username"]
    if not username or not cleaned["email"]:
        return jsonify({"error": "username and email required"}), 400
    if username in MEMBERS:
        return jsonify({"error": "Username already exists"}), 409
    MEMBERS[username] = cleaned  # vuln-code-snippet vuln-line chain_role_param_vuln
    return jsonify({"status": "created", "member": cleaned}), 201
# vuln-code-snippet end chain_role_param_vuln


if __name__ == "__main__":
    app.run(port=5000)
