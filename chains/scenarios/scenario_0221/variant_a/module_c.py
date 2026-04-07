import json
from flask import Blueprint, request, jsonify
from module_b import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/graphql", methods=["POST"])
def graphql_endpoint():
    body = request.get_json(silent=True) or {}
    query = body.get("query", "")
    result = {"data": {"viewer": {"id": "u1", "secret": "my-private-key"}}}
    return jsonify(result)
