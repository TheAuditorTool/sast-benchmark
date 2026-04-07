"""Billing plan routes.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. PUT /accounts/<id>/plan passes the full body to deserialize_plan_update.
  2. The returned dict may include 'tier' set to any value by the caller.
  3. The account is updated without payment verification.

CWE-915: Mass assignment of billing tier enables free plan upgrade.
"""
import functools
from flask import request, jsonify
from models import app, ACCOUNTS
from serializers import deserialize_plan_update


def _require_auth(f):
    """Require X-Account-Id header matching the route parameter."""
    @functools.wraps(f)
    def decorated(account_id, *args, **kwargs):
        if request.headers.get("X-Account-Id") != account_id:
            return jsonify({"error": "Forbidden"}), 403
        return f(account_id, *args, **kwargs)
    return decorated


@app.route("/accounts/<account_id>/plan", methods=["PUT"])
@_require_auth
def update_plan(account_id):
    """Update billing plan for an account."""
    if account_id not in ACCOUNTS:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    updates = deserialize_plan_update(data)
    ACCOUNTS[account_id].update(updates)
    return jsonify({"status": "updated", "tier": ACCOUNTS[account_id]["tier"]})


if __name__ == "__main__":
    app.run(port=5000)
