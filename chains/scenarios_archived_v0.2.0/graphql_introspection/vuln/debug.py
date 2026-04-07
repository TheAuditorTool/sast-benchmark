"""GraphQL introspection endpoint -- VULNERABLE variant.

Serves the full schema including admin-only mutation names and their
argument shapes, letting an attacker discover privileged operations
without any authentication.

CWE-200: Exposure of Sensitive Information
Chain:
  1. GET /graphql/schema exposes the resetUserPassword mutation and its token argument.
  2. An attacker uses the schema to craft a valid mutation using a known reset_token.
"""
from flask import jsonify
from config import app

SCHEMA = {
    "queries": ["getUser", "listPublicPosts"],
    "mutations": [
        {"name": "updateProfile",     "args": ["userId", "email"]},
        {"name": "resetUserPassword", "args": ["username", "token", "newPassword"]},
        {"name": "deleteUser",        "args": ["userId", "adminToken"]},
        {"name": "grantAdminRole",    "args": ["userId", "adminToken"]},
    ],
}


# vuln-code-snippet start chain_graphql_introspect_vuln
@app.route("/graphql/schema")
def graphql_schema():
    """Return the full GraphQL schema including admin mutations.

    VULNERABLE: no authentication required; admin mutation names and
    their argument lists are exposed to any caller.
    """
    return jsonify({"schema": SCHEMA})  # vuln-code-snippet vuln-line chain_graphql_introspect_vuln
# vuln-code-snippet end chain_graphql_introspect_vuln
