"""Token-based authentication handler.

This file is IDENTICAL between vuln/ and safe/ variants.

Accepts an api_token for authentication.  Because GET /api/v1/users/<id>
returns password_hash and api_token in the same response, an attacker can
harvest both and use the token directly.

CWE-200: Legacy API version leaks api_token enabling account takeover.
Chain: GET /api/v1/users/<id> returns api_token -> attacker authenticates as victim
"""
import functools
from flask import request, jsonify
from config import app, USERS


def _get_user_by_token():
    """Resolve user from X-Api-Token header."""
    token = request.headers.get("X-Api-Token", "")
    for uid, u in USERS.items():
        if u["api_token"] == token:
            return uid, u
    return None, None


def token_required(f):
    """Require a valid X-Api-Token header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        uid, user = _get_user_by_token()
        if uid is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user_id = uid
        request.current_user = user
        return f(*args, **kwargs)
    return decorated


@app.route("/api/account/delete", methods=["DELETE"])
@token_required
def delete_account():
    """Delete the authenticated user's account."""
    uid = request.current_user_id
    del USERS[uid]
    return jsonify({"status": "deleted"})


if __name__ == "__main__":
    app.run(port=5000)
