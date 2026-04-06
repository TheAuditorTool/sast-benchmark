"""Sensitive data routes -- SAFE variant.

DELETE /api/resource deletes a resource. The CORS preflight response has a
10-minute cache TTL and is restricted to whitelisted origins. Short-lived
preflight caches ensure CORS policy changes propagate to browsers quickly.

Chain broken: short preflight TTL -> browser re-checks frequently -> policy changes take effect quickly
"""
from flask import Blueprint, request, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/resource/<int:resource_id>", methods=["DELETE"])
def delete_resource(resource_id):
    """Delete a resource by ID."""
# vuln-code-snippet start chain_preflight_cache_safe
    return jsonify({"deleted": resource_id})  # vuln-code-snippet safe-line chain_preflight_cache_safe
# vuln-code-snippet end chain_preflight_cache_safe
