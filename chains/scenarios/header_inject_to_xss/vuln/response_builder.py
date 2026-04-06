"""Response header builder -- VULNERABLE variant.

GET /api/set-theme accepts a user-supplied 'theme' value and sets it
as a cookie without stripping CRLF characters. An attacker can inject
\r\n sequences to add arbitrary HTTP headers or split the response,
setting a crafted Set-Cookie header whose value contains an XSS payload
that the dashboard renders unescaped.

Chain: CRLF injection sets malicious cookie -> dashboard renders cookie unescaped
Individual findings: header injection (medium)
Chain finding: CRLF-injected cookie value causes XSS on dashboard (high)
"""
from flask import Blueprint, request, make_response, jsonify

response_builder_bp = Blueprint("response_builder", __name__)


# vuln-code-snippet start chain_header_xss_vuln
@response_builder_bp.route("/api/set-theme")
def set_theme():
    """Set user theme preference as a cookie."""
    theme = request.args.get("theme", "light")
    response = make_response(jsonify({"status": "ok", "theme": theme}))
    response.headers["Set-Cookie"] = "theme=" + theme + "; Path=/"  # vuln-code-snippet vuln-line chain_header_xss_vuln
    return response
# vuln-code-snippet end chain_header_xss_vuln
