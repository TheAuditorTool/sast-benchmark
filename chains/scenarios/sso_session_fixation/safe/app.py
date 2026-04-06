"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts SSO endpoints and the protected portal that requires authentication.
"""
from flask import jsonify, request
from auth import app
from session import require_auth


@app.route("/portal")
@require_auth
def portal():
    """Return portal content for the authenticated SSO user."""
    return jsonify({
        "user": request.session["user_id"],
        "email": request.session.get("email"),
        "role": request.session["role"],
    })


if __name__ == "__main__":
    app.run(port=5007)
