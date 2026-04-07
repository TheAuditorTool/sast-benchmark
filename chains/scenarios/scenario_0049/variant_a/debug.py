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

# vuln-code-snippet start ChainScenario0049A
@app.route("/swagger.json")
def swagger():
    return jsonify(OPENAPI_SPEC)  # vuln-code-snippet target-line ChainScenario0049A
# vuln-code-snippet end ChainScenario0049A
