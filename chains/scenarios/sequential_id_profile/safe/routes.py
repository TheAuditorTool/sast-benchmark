"""Flask routes for the user profile API -- SAFE variant.

The GET /api/users/<user_id>/profile endpoint verifies that the
authenticated user's session id matches the requested user_id before
fetching any data.  Sequential enumeration is therefore harmless.

Chain: broken -- ownership check enforced at the retrieval call site
CWE-862: Missing Authorization (remediated)
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

    FIXED: the route verifies that the requesting user owns the
    profile before fetching it.  A user requesting another user's
    id receives 403.
    """
    current_user = get_current_user()
    if current_user != user_id:
        return jsonify({"error": "Access denied"}), 403

# vuln-code-snippet start chain_seq_id_profile_safe
    profile = get_profile_by_id(user_id)  # vuln-code-snippet safe-line chain_seq_id_profile_safe
# vuln-code-snippet end chain_seq_id_profile_safe
    if profile is None:
        return jsonify({"error": "User not found"}), 404
    return jsonify(profile)


if __name__ == "__main__":
    app.run(port=5000)
