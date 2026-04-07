"""Public Swagger spec endpoint -- SAFE variant.

Only serves documentation for public endpoints.  Admin endpoint paths,
required headers, and parameter names are excluded from the spec.

CWE-200: Fixed by removing admin endpoint definitions from the public spec.
Chain: GET /swagger.json -> admin paths absent -> attacker cannot discover admin endpoints
"""
from flask import jsonify
from config import app

PUBLIC_SPEC = {
    "openapi": "3.0.0",
    "info": {"title": "App API", "version": "1.0"},
    "paths": {
        "/api/users": {
            "get": {"summary": "List users", "security": [{"bearerAuth": []}]},
        },
    },
}


# vuln-code-snippet start chain_swagger_leak_safe
@app.route("/swagger.json")
def swagger():
    """Serve only the public OpenAPI spec.

    SAFE: admin endpoint paths and authentication details are not included.
    """
    return jsonify(PUBLIC_SPEC)  # vuln-code-snippet safe-line chain_swagger_leak_safe
# vuln-code-snippet end chain_swagger_leak_safe
