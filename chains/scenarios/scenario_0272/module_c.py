from flask import jsonify
from module_b import app

PUBLIC_SPEC = {
    "openapi": "3.0.0",
    "info": {"title": "App API", "version": "1.0"},
    "paths": {
        "/api/users": {
            "get": {"summary": "List users", "security": [{"bearerAuth": []}]},
        },
    },
}

@app.route("/swagger.json")
def swagger():
    return jsonify(PUBLIC_SPEC)
