from flask import request, jsonify
from module_b import app, USERS

PUBLIC_SCHEMA = {
    "queries": ["getUser", "listPublicPosts"],
}

@app.route("/graphql/schema")
def graphql_schema():
    caller = request.headers.get("X-User-Id", "")
    if USERS.get(caller, {}).get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"schema": PUBLIC_SCHEMA})
