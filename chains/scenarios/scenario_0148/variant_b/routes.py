from flask import Blueprint, request, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/resource/<int:resource_id>", methods=["DELETE"])
def delete_resource(resource_id):
# vuln-code-snippet start ChainScenario0148B
    return jsonify({"deleted": resource_id})  # vuln-code-snippet target-line ChainScenario0148B
# vuln-code-snippet end ChainScenario0148B
