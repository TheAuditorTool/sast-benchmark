"""Signup and team views -- VULNERABLE variant.

/signup accepts any POST body and creates an account without validating
the invite token field, allowing any guest to register.

Chain: guest POST /signup -> invite check absent -> user created
"""
from flask import request, jsonify
from models import app, USERS
from auth import validate_invite, consume_invite


@app.route("/health")
def health():
    """Health check endpoint."""
    return jsonify({"status": "ok"})


# vuln-code-snippet start chain_guest_signup_vuln
@app.route("/signup", methods=["POST"])
def signup():
    """Register a new user. VULNERABLE: invite token is not validated."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "").strip()
    email = data.get("email", "").strip()
    if not username or not email:
        return jsonify({"error": "username and email required"}), 400
    if username in USERS:
        return jsonify({"error": "Username taken"}), 409
    USERS[username] = {"email": email, "role": "user"}  # vuln-code-snippet vuln-line chain_guest_signup_vuln
    return jsonify({"status": "registered", "username": username}), 201
# vuln-code-snippet end chain_guest_signup_vuln


@app.route("/team/members", methods=["GET"])
def team_members():
    """List registered team members. Authenticated endpoint."""
    return jsonify({"members": list(USERS.keys())})


if __name__ == "__main__":
    app.run(port=5000)
