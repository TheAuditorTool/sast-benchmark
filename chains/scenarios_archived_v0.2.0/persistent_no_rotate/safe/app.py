"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts authentication routes and a protected settings endpoint.
"""
from flask import jsonify, request
from auth import app
from session_store import require_auth


@app.route("/settings")
@require_auth
def settings():
    """Return account settings for the authenticated user."""
    return jsonify({
        "user": request.session["user_id"],
        "role": request.session["role"],
    })


if __name__ == "__main__":
    app.run(port=5005)
