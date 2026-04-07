"""Resource management routes -- SAFE variant.

GET /api/item/<id> reads an item. DELETE operations require an actual DELETE
request, which triggers a CORS preflight enforced against the allowlist.
The method-override header is no longer honored, closing the bypass.

Chain broken: no method override -> DELETE requires actual method + preflight -> trusted origin only
"""
from flask import Blueprint, request, jsonify
from middleware import apply_cors, get_effective_method

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/item/<int:item_id>", methods=["GET"])
def manage_item(item_id):
    """Read an item; override header is ignored in the safe variant."""
    method = get_effective_method()
# vuln-code-snippet start chain_method_override_safe
    if method == "DELETE":
        return jsonify({"deleted": item_id})  # vuln-code-snippet safe-line chain_method_override_safe
# vuln-code-snippet end chain_method_override_safe
    return jsonify({"id": item_id, "name": "Widget"})
