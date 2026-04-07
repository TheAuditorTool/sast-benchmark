"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts the init/login/logout endpoints and a protected resource.
"""
from flask import jsonify, request
from auth import app
from session import require_auth


@app.route("/secure-data")
@require_auth
def secure_data():
    """Return sensitive data only available over authenticated sessions."""
    return jsonify({
        "user": request.session["user_id"],
        "secret": "sensitive-value-42",
    })


if __name__ == "__main__":
    app.run(port=5006)
