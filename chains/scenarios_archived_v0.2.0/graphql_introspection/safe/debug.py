"""GraphQL introspection endpoint -- SAFE variant.

Introspection is disabled in production.  The schema endpoint requires
admin authentication and only returns public query names, not admin
mutations or their argument shapes.

CWE-200: Fixed by disabling unauthenticated schema introspection.
Chain: GET /graphql/schema -> 403 (auth required) -> admin mutations not revealed
"""
from flask import request, jsonify
from config import app, USERS

PUBLIC_SCHEMA = {
    "queries": ["getUser", "listPublicPosts"],
}


# vuln-code-snippet start chain_graphql_introspect_safe
@app.route("/graphql/schema")
def graphql_schema():
    """Return only the public schema fragment; requires admin auth.

    SAFE: admin mutations are not included; endpoint requires authentication.
    """
    caller = request.headers.get("X-User-Id", "")
    if USERS.get(caller, {}).get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"schema": PUBLIC_SCHEMA})  # vuln-code-snippet safe-line chain_graphql_introspect_safe
# vuln-code-snippet end chain_graphql_introspect_safe
