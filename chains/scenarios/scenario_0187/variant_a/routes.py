from flask import Blueprint, request, jsonify
from middleware import apply_cors, get_effective_method

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/item/<int:item_id>", methods=["GET"])
def manage_item(item_id):
    method = get_effective_method()
# vuln-code-snippet start ChainScenario0187A
    if method == "DELETE":
        return jsonify({"deleted": item_id})  # vuln-code-snippet target-line ChainScenario0187A
# vuln-code-snippet end ChainScenario0187A
    return jsonify({"id": item_id, "name": "Widget"})
