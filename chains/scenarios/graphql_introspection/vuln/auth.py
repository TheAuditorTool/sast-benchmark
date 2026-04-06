"""GraphQL mutation executor.

This file is IDENTICAL between vuln/ and safe/ variants.

Executes the resetUserPassword mutation discovered via the introspection
endpoint.  Any caller who knows the mutation signature and a valid token
can reset any user's password.

CWE-200: Schema introspection reveals mutation to enable account takeover.
Chain: GET /graphql/schema reveals mutation -> attacker crafts reset mutation -> account taken over
"""
import functools
from flask import request, jsonify
from config import app, USERS


def _require_auth(f):
    """Require X-User-Id header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if not request.headers.get("X-User-Id"):
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated


@app.route("/graphql", methods=["POST"])
def graphql_endpoint():
    """Execute a GraphQL-style mutation."""
    body = request.get_json(force=True) or {}
    operation = body.get("operationName", "")
    variables = body.get("variables", {})

    if operation == "resetUserPassword":
        username = variables.get("username", "")
        token = variables.get("token", "")
        new_password = variables.get("newPassword", "")
        user = USERS.get(username)
        if user is None:
            return jsonify({"errors": [{"message": "User not found"}]}), 404
        if user.get("reset_token") != token or token is None:
            return jsonify({"errors": [{"message": "Invalid token"}]}), 401
        user["password"] = new_password
        user["reset_token"] = None
        return jsonify({"data": {"resetUserPassword": {"success": True}}})

    return jsonify({"errors": [{"message": "Unknown operation"}]}), 400


if __name__ == "__main__":
    app.run(port=5000)
