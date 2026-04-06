"""Internal API routes -- VULNERABLE variant.

GET /internal/metrics returns service health metrics and internal counters
that should only be accessible server-side. The open CORS policy on this
blueprint lets any browser-based client from any origin read the response.

Chain: open CORS on internal API -> any browser can fetch metrics -> internal data leaked
Individual findings: internal metrics exposed via CORS (CWE-942)
Chain finding: service metric exfiltration via open CORS on internal endpoint (high)
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/internal/metrics", methods=["GET"])
def get_metrics():
    """Return internal service health metrics."""
# vuln-code-snippet start chain_internal_api_cors_vuln
    payload = {
        "db_connections": 42,
        "queue_depth": 7,
        "internal_ip": "10.0.1.50",  # vuln-code-snippet vuln-line chain_internal_api_cors_vuln
    }
# vuln-code-snippet end chain_internal_api_cors_vuln
    return jsonify(payload)
