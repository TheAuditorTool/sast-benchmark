"""Sensitive data routes -- VULNERABLE variant.

DELETE /api/resource deletes a resource by ID. The CORS preflight response
has a 24-hour cache TTL. If the attacker can poison the preflight cache,
the browser will not re-check permissions for 24 hours, extending the window
during which a CORS policy change would not take effect.

Chain: 24h preflight cache -> policy change not re-checked -> stale permissions exploited
Individual findings: long-lived preflight cache on destructive endpoint (CWE-942)
Chain finding: CORS permission window extension via preflight cache poisoning (medium)
"""
from flask import Blueprint, request, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/resource/<int:resource_id>", methods=["DELETE"])
def delete_resource(resource_id):
    """Delete a resource by ID."""
# vuln-code-snippet start chain_preflight_cache_vuln
    return jsonify({"deleted": resource_id})  # vuln-code-snippet vuln-line chain_preflight_cache_vuln
# vuln-code-snippet end chain_preflight_cache_vuln
