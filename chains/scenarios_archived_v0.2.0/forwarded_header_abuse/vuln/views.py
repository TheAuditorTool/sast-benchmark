"""Views -- VULNERABLE variant for forwarded_header_abuse.

GET /reset-link returns a cached password-reset link whose base URL
is derived from the Forwarded header.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, jsonify
from response_builder import base_url_from_forwarded

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_forwarded_abuse_vuln
@views_bp.route("/reset-link")
def reset_link():
    """Return a cached password-reset link."""
    token = request.args.get("token", "abc123")
    base = base_url_from_forwarded()
    link = f"{base}/reset?token={token}"
    from flask import make_response
    resp = make_response(jsonify({"link": link}))
    resp.headers["Cache-Control"] = "public, max-age=60"
    return resp  # vuln-code-snippet vuln-line chain_forwarded_abuse_vuln
# vuln-code-snippet end chain_forwarded_abuse_vuln
