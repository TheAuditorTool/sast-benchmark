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

# vuln-code-snippet start ChainScenario0049B
@app.route("/swagger.json")
def swagger():
    return jsonify(PUBLIC_SPEC)  # vuln-code-snippet target-line ChainScenario0049B
# vuln-code-snippet end ChainScenario0049B
