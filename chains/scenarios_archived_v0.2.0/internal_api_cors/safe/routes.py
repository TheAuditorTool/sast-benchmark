"""Internal API routes -- SAFE variant.

GET /internal/metrics returns service health metrics. No CORS headers are
emitted, so browsers block cross-origin reads and the data is only accessible
to same-origin or server-side callers.

Chain broken: no CORS headers emitted -> browser blocks cross-origin reads -> internal data protected
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/internal/metrics", methods=["GET"])
def get_metrics():
    """Return internal service health metrics."""
# vuln-code-snippet start chain_internal_api_cors_safe
    payload = {
        "db_connections": 42,
        "queue_depth": 7,
        "internal_ip": "10.0.1.50",  # vuln-code-snippet safe-line chain_internal_api_cors_safe
    }
# vuln-code-snippet end chain_internal_api_cors_safe
    return jsonify(payload)
