"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts the landing page, auth endpoints, and a protected orders endpoint.
"""
from flask import jsonify, request
from auth import app
from session import require_auth


@app.route("/orders")
@require_auth
def orders():
    """Return order history for the authenticated user."""
    return jsonify({
        "user": request.session["user_id"],
        "orders": ["order-1", "order-2"],
    })


if __name__ == "__main__":
    app.run(port=5012)
