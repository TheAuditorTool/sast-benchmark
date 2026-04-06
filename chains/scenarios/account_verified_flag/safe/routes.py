"""Account update routes.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. PATCH /account passes the full body to deserialize_account_update.
  2. The returned dict may include 'email_verified': True.
  3. The user bypasses the email verification gate without clicking the link.

CWE-915: Mass assignment of email_verified flag bypasses verification.
"""
import functools
from flask import request, jsonify
from models import app, USERS
from serializers import deserialize_account_update


def _require_auth(f):
    """Require X-User-Id header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-User-Id") not in USERS:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated


@app.route("/account", methods=["PATCH"])
@_require_auth
def update_account():
    """Update the caller's account."""
    user_id = request.headers.get("X-User-Id")
    data = request.get_json(force=True) or {}
    updates = deserialize_account_update(data)
    USERS[user_id].update(updates)
    return jsonify({"status": "updated", "user": USERS[user_id]})


if __name__ == "__main__":
    app.run(port=5000)
