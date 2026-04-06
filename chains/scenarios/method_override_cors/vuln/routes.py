"""Resource management routes -- VULNERABLE variant.

GET /api/item/<id> supports reading and, via X-HTTP-Method-Override, deletion.
Because GET is a simple CORS method (no preflight), an attacker's page can send
a credentialed GET with X-HTTP-Method-Override: DELETE and delete a victim's
items without triggering a preflight check.

Chain: GET + override -> DELETE executed cross-origin -> resource deleted without preflight
Individual findings: destructive method accessible via CORS simple-method override (CWE-942)
Chain finding: unauthorized resource deletion via CORS preflight bypass (high)
"""
from flask import Blueprint, request, jsonify
from middleware import apply_cors, get_effective_method

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/item/<int:item_id>", methods=["GET"])
def manage_item(item_id):
    """Read or delete an item depending on the effective HTTP method."""
    method = get_effective_method()
# vuln-code-snippet start chain_method_override_vuln
    if method == "DELETE":
        return jsonify({"deleted": item_id})  # vuln-code-snippet vuln-line chain_method_override_vuln
# vuln-code-snippet end chain_method_override_vuln
    return jsonify({"id": item_id, "name": "Widget"})
