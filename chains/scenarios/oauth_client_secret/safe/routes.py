"""Flask route registration for oauth_client_secret scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, jsonify

routes_bp = Blueprint("routes", __name__)


@routes_bp.route("/health", methods=["GET"])
def health():
    """Return application health status."""
    return jsonify({"status": "ok"})
