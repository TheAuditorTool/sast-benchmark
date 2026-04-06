"""Signup and team views -- SAFE variant.

/signup now validates the invite token before creating an account.
A guest without a valid token receives 403 and cannot register.

Chain: guest POST /signup -> invite validated -> 403 if no valid token (chain broken)
"""
from flask import request, jsonify
from models import app, USERS
from auth import validate_invite, consume_invite


@app.route("/health")
def health():
    """Health check endpoint."""
    return jsonify({"status": "ok"})


# vuln-code-snippet start chain_guest_signup_safe
@app.route("/signup", methods=["POST"])
def signup():
    """Register a new user. SAFE: invite token is validated before account creation."""
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
    USERS[username] = {"email": email, "role": "user"}  # vuln-code-snippet safe-line chain_guest_signup_safe
    return jsonify({"status": "registered", "username": username}), 201
# vuln-code-snippet end chain_guest_signup_safe


@app.route("/team/members", methods=["GET"])
def team_members():
    """List registered team members. Authenticated endpoint."""
    return jsonify({"members": list(USERS.keys())})


if __name__ == "__main__":
    app.run(port=5000)
