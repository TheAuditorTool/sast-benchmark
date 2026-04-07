import json
from flask import Blueprint, request, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/graphql", methods=["POST"])
def graphql_endpoint():
    body = request.get_json(silent=True) or {}
    query = body.get("query", "")
# vuln-code-snippet start ChainScenario0221A
    result = {"data": {"viewer": {"id": "u1", "secret": "my-private-key"}}}  # vuln-code-snippet target-line ChainScenario0221A
# vuln-code-snippet end ChainScenario0221A
    return jsonify(result)
