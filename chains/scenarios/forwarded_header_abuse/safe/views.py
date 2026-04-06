"""Views -- SAFE variant for forwarded_header_abuse.

GET /reset-link returns a cached password-reset link whose base URL
is the trusted hard-coded origin.

This file is IDENTICAL between vuln/ and safe/ variants (only
response_builder.py changes).
"""
from flask import Blueprint, request, jsonify
from response_builder import base_url_from_forwarded

views_bp = Blueprint("views", __name__)


# vuln-code-snippet start chain_forwarded_abuse_safe
@views_bp.route("/reset-link")
def reset_link():
    """Return a cached password-reset link with a trusted base URL."""
    token = request.args.get("token", "abc123")
    base = base_url_from_forwarded()
    link = f"{base}/reset?token={token}"
    from flask import make_response
    resp = make_response(jsonify({"link": link}))
    resp.headers["Cache-Control"] = "public, max-age=60"
    return resp  # vuln-code-snippet safe-line chain_forwarded_abuse_safe
# vuln-code-snippet end chain_forwarded_abuse_safe
