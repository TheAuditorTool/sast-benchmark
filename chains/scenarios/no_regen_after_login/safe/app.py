"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts the auth blueprint and a protected profile endpoint that an attacker
would target after gaining authenticated access via session fixation.
"""
from flask import jsonify
from auth import app
from session import require_auth


@app.route("/profile")
@require_auth
def profile():
    """Return the authenticated user's profile data."""
    return jsonify({
        "user_id": request.session["user_id"],
        "role":    request.session["role"],
    })


from flask import request  # noqa: E402 -- needed after app import

if __name__ == "__main__":
    app.run(port=5001)
