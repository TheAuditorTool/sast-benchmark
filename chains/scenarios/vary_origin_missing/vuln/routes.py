"""Sensitive data routes -- VULNERABLE variant.

GET /api/documents returns a list of documents the user has access to.
The CORS middleware omits Vary: Origin, so a caching layer could serve a
response that contains an ACAO header for a different origin to an origin
that should be denied, enabling unintended cross-origin reads.

Chain: missing Vary: Origin -> CDN stores wrong ACAO -> unapproved origin reads documents
Individual findings: document list exposed under CORS response without Vary: Origin (CWE-942)
Chain finding: document exfiltration via cache-confused CORS policy (medium)
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/documents", methods=["GET"])
def get_documents():
    """Return the list of documents accessible to the current user."""
# vuln-code-snippet start chain_vary_origin_vuln
    payload = {
        "documents": [
            {"id": 1, "title": "Q1 Report", "confidential": True},  # vuln-code-snippet vuln-line chain_vary_origin_vuln
        ]
    }
# vuln-code-snippet end chain_vary_origin_vuln
    return jsonify(payload)
