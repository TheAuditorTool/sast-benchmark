"""GraphQL endpoint -- SAFE variant.

POST /graphql handles GraphQL queries. The CORS middleware restricts access
to the whitelisted origin, so attacker-controlled sites cannot make
credentialed cross-origin GraphQL queries.

Chain broken: only one origin allowed -> attacker origin rejected -> GraphQL inaccessible cross-origin
"""
import json
from flask import Blueprint, request, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/graphql", methods=["POST"])
def graphql_endpoint():
    """Execute a simplified GraphQL query and return results."""
    body = request.get_json(silent=True) or {}
    query = body.get("query", "")
# vuln-code-snippet start chain_graphql_cors_safe
    result = {"data": {"viewer": {"id": "u1", "secret": "my-private-key"}}}  # vuln-code-snippet safe-line chain_graphql_cors_safe
# vuln-code-snippet end chain_graphql_cors_safe
    return jsonify(result)
