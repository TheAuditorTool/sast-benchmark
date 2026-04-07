"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts the auth and form endpoints, plus a protected action endpoint.
"""
from flask import jsonify, request
from auth import app
from session import require_auth


@app.route("/action", methods=["POST"])
@require_auth
def action():
    """Execute a protected action for the authenticated user."""
    return jsonify({
        "result": "action executed",
        "user": request.session["user_id"],
    })


if __name__ == "__main__":
    app.run(port=5004)
