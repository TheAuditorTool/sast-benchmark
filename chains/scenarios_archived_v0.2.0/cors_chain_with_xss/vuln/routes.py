"""Sensitive data routes -- VULNERABLE variant.

GET /api/private-notes returns private user notes. The CORS middleware
trusts feedback.example.com, which has a stored XSS flaw. An attacker who
injects script into the feedback subdomain can pivot to this endpoint and
exfiltrate private notes from a victim's authenticated session.

Chain: XSS payload at feedback.example.com -> fetches /api/private-notes with victim credentials -> notes stolen
Individual findings: private data exposed under CORS trusting XSS-vulnerable origin (CWE-942)
Chain finding: private data exfiltration via XSS-chained CORS attack (critical)
"""
from flask import Blueprint, jsonify
from middleware import apply_cors

routes_bp = Blueprint("routes", __name__)

routes_bp.after_request(apply_cors)


@routes_bp.route("/api/private-notes", methods=["GET"])
def get_notes():
    """Return the current user's private notes."""
# vuln-code-snippet start chain_cors_xss_vuln
    payload = {
        "notes": [
            {"id": 1, "body": "meeting at 3pm - confidential details"},  # vuln-code-snippet vuln-line chain_cors_xss_vuln
        ]
    }
# vuln-code-snippet end chain_cors_xss_vuln
    return jsonify(payload)
