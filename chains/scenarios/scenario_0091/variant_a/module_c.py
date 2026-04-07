from flask import jsonify
from module_b import app

SCHEMA = {
    "queries": ["getUser", "listPublicPosts"],
    "mutations": [
        {"name": "updateProfile",     "args": ["userId", "email"]},
        {"name": "resetUserPassword", "args": ["username", "token", "newPassword"]},
        {"name": "deleteUser",        "args": ["userId", "adminToken"]},
        {"name": "grantAdminRole",    "args": ["userId", "adminToken"]},
    ],
}

@app.route("/graphql/schema")
def graphql_schema():
    return jsonify({"schema": SCHEMA})
