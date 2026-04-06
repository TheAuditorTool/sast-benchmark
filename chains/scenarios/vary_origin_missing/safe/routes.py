"""Sensitive data routes -- SAFE variant.

GET /api/documents returns document listings. The CORS middleware now
includes Vary: Origin, so caches store separate response variants per
origin and cannot serve one origin's ACAO to another.

Chain broken: Vary: Origin present -> cache stores per-origin responses -> ACAO headers not confused
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/documents", methods=["GET"])
def get_documents():
    """Return the list of documents accessible to the current user."""
# vuln-code-snippet start chain_vary_origin_safe
    payload = {
        "documents": [
            {"id": 1, "title": "Q1 Report", "confidential": True},  # vuln-code-snippet safe-line chain_vary_origin_safe
        ]
    }
# vuln-code-snippet end chain_vary_origin_safe
    return jsonify(payload)
