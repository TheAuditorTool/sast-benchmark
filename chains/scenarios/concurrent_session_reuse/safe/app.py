"""Flask application entry point -- IDENTICAL between vuln/ and safe/ variants.

Mounts login/logout and a protected inbox endpoint that an attacker would
access using a stolen session token.
"""
from flask import jsonify, request
from auth import app
from session import require_auth


@app.route("/inbox")
@require_auth
def inbox():
    """Return messages for the authenticated user."""
    return jsonify({
        "user": request.session["user_id"],
        "messages": ["msg-1", "msg-2"],
    })


if __name__ == "__main__":
    app.run(port=5011)
