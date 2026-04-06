"""GraphQL endpoint -- VULNERABLE variant.

POST /graphql handles GraphQL queries. The open CORS policy lets any website
make credentialed queries. An attacker can run introspection to map the full
schema and then query sensitive fields on behalf of an authenticated victim.

Chain: any origin + credentials -> run introspection -> discover schema -> query sensitive data
Individual findings: authenticated GraphQL data exposed via open CORS (CWE-942)
Chain finding: full schema + data exfiltration via open CORS on GraphQL (critical)
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
# vuln-code-snippet start chain_graphql_cors_vuln
    result = {"data": {"viewer": {"id": "u1", "secret": "my-private-key"}}}  # vuln-code-snippet vuln-line chain_graphql_cors_vuln
# vuln-code-snippet end chain_graphql_cors_vuln
    return jsonify(result)
