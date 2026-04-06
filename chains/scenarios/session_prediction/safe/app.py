"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts login/logout and a protected admin panel accessible to any user
whose predicted session token passes validation.
"""
from flask import jsonify, request
from auth import app
from session import require_auth


@app.route("/admin")
@require_auth
def admin():
    """Return admin panel data -- sensitive endpoint that makes prediction worthwhile."""
    return jsonify({
        "user": request.session["user_id"],
        "role": request.session["role"],
        "admin_data": "internal-config-42",
    })


if __name__ == "__main__":
    app.run(port=5010)
