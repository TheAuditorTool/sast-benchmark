from flask import request, jsonify
from config import app, USERS

PUBLIC_SCHEMA = {
    "queries": ["getUser", "listPublicPosts"],
}

# vuln-code-snippet start ChainScenario0091B
@app.route("/graphql/schema")
def graphql_schema():
    caller = request.headers.get("X-User-Id", "")
    if USERS.get(caller, {}).get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"schema": PUBLIC_SCHEMA})  # vuln-code-snippet target-line ChainScenario0091B
# vuln-code-snippet end ChainScenario0091B
