from flask import Blueprint, request, jsonify
from module_b import apply_cors, get_effective_method

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)

@routes_bp.route("/api/item/<int:item_id>", methods=["GET"])
def manage_item(item_id):
    method = get_effective_method()
    if method == "DELETE":
        return jsonify({"deleted": item_id})
    return jsonify({"id": item_id, "name": "Widget"})
