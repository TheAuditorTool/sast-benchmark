"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts registration/revocation endpoints and a protected API resource.
"""
from flask import jsonify, request
from auth import app
from session import require_api_auth


@app.route("/api/resource")
@require_api_auth
def resource():
    """Return a protected resource for the authenticated API caller."""
    return jsonify({
        "user": request.api_record["user_id"],
        "data": "protected-api-data",
    })


if __name__ == "__main__":
    app.run(port=5009)
