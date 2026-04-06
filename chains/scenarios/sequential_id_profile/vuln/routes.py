"""Flask routes for the user profile API -- VULNERABLE variant.

The GET /api/users/<user_id>/profile endpoint fetches a full profile
record using only the URL parameter.  Because sequential integer IDs
are trivially enumerable and no ownership check is performed, any
authenticated user can read any other user's profile (email, phone,
address, bio).

Chain: authenticated session + sequential id -> unowned profile data
Individual findings: missing ownership check (medium)
Chain finding: mass data exfiltration of all user PII (high)
CWE-862: Missing Authorization
"""
from flask import Flask, jsonify, session
from auth import login_required, get_current_user
from models import get_profile_by_id

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"


@app.route("/api/session/login", methods=["POST"])
def login():
    """Minimal login stub that sets a session for testing."""
    from flask import request
    data = request.get_json(force=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    session["user_id"] = int(user_id)
    return jsonify({"ok": True})


@app.route("/api/users/<int:user_id>/profile")
@login_required
def get_user_profile(user_id):
    """Return a user's full profile.

    VULNERABLE: the route accepts any user_id without verifying that
    the requesting user owns that profile.  An attacker can walk
    user_id = 1, 2, 3 ... and harvest all user PII.
    """
# vuln-code-snippet start chain_seq_id_profile_vuln
    profile = get_profile_by_id(user_id)  # vuln-code-snippet vuln-line chain_seq_id_profile_vuln
# vuln-code-snippet end chain_seq_id_profile_vuln
    if profile is None:
        return jsonify({"error": "User not found"}), 404
    return jsonify(profile)


if __name__ == "__main__":
    app.run(port=5000)
