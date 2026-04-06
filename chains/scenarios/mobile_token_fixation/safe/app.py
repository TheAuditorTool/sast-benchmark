"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts device auth endpoints and a protected data endpoint.
"""
from flask import jsonify, request
from auth import app
from session import require_auth


@app.route("/api/data")
@require_auth
def data():
    """Return protected user data for the authenticated device."""
    return jsonify({
        "user": request.token_record["user_id"],
        "device": request.token_record["device_id"],
    })


if __name__ == "__main__":
    app.run(port=5008)
