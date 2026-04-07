"""Authentication / privileged action handler.

This file is IDENTICAL between vuln/ and safe/ variants.

Any code path that raises an exception will have its error handler expose
STRIPE_API_KEY via errors.py.  This file deliberately triggers a division
by zero to demonstrate a realistic error path.

CWE-200: Verbose error response leaks API keys enabling account takeover.
Chain: GET /crash -> exception -> error handler returns STRIPE_API_KEY -> attacker uses key
"""
import functools
from flask import request, jsonify
from errors import app
from config import STRIPE_API_KEY


def _api_key_required(f):
    """Require X-Api-Key header matching STRIPE_API_KEY."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-Api-Key") != STRIPE_API_KEY:
            return jsonify({"error": "Invalid API key"}), 401
        return f(*args, **kwargs)
    return decorated


@app.route("/crash")
def crash():
    """Intentionally crash to trigger the error handler."""
    return str(1 // 0)


@app.route("/billing/charge", methods=["POST"])
@_api_key_required
def charge():
    """Process a charge using the Stripe key."""
    amount = request.get_json(force=True).get("amount", 0)
    return jsonify({"charged": amount})


if __name__ == "__main__":
    app.run(port=5000)
