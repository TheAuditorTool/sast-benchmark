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

# vuln-code-snippet start ChainScenario0091A
@app.route("/graphql/schema")
def graphql_schema():
    return jsonify({"schema": SCHEMA})  # vuln-code-snippet target-line ChainScenario0091A
# vuln-code-snippet end ChainScenario0091A
