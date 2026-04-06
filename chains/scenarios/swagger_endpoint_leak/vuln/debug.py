"""Public Swagger spec endpoint -- VULNERABLE variant.

Serves the full OpenAPI specification including the admin endpoint paths,
their required parameters, and the X-Admin-Token header name, enabling an
attacker to enumerate privileged operations and the token they require.

CWE-200: Exposure of Sensitive Information
Chain:
  1. GET /swagger.json reveals POST /admin/create-user and its X-Admin-Token header.
  2. Attacker learns the header name and guesses or brute-forces the token.
"""
from flask import jsonify
from config import app

OPENAPI_SPEC = {
    "openapi": "3.0.0",
    "info": {"title": "App API", "version": "1.0"},
    "paths": {
        "/api/users": {
            "get": {"summary": "List users", "security": [{"bearerAuth": []}]},
        },
        "/admin/create-user": {
            "post": {
                "summary": "Create a user (admin only)",
                "parameters": [{"name": "X-Admin-Token", "in": "header", "required": True}],
            },
        },
        "/admin/delete-user": {
            "delete": {
                "summary": "Delete a user (admin only)",
                "parameters": [{"name": "X-Admin-Token", "in": "header", "required": True}],
            },
        },
    },
}


# vuln-code-snippet start chain_swagger_leak_vuln
@app.route("/swagger.json")
def swagger():
    """Serve the full OpenAPI spec including admin endpoint details.

    VULNERABLE: admin paths and their required authentication headers are
    exposed to unauthenticated callers.
    """
    return jsonify(OPENAPI_SPEC)  # vuln-code-snippet vuln-line chain_swagger_leak_vuln
# vuln-code-snippet end chain_swagger_leak_vuln
