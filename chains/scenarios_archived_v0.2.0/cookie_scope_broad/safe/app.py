"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Hosts the main application at app.example.com.  The session cookie domain
controls whether subdomains can also read the session token.
"""
from flask import jsonify, request
from auth import app
from session import require_auth


@app.route("/account")
@require_auth
def account():
    """Return account details for the authenticated user."""
    return jsonify({
        "user": request.session["user_id"],
        "role": request.session["role"],
    })


if __name__ == "__main__":
    app.run(port=5003)
