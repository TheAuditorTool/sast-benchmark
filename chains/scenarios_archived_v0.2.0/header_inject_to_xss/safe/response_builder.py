"""Response header builder -- SAFE variant.

GET /api/set-theme strips all CR and LF characters from the user-supplied
theme value before writing it to the Set-Cookie header. CRLF injection
is prevented, so no attacker-controlled header splitting can occur and
no malicious cookie value reaches the dashboard.

Chain broken: CRLF stripped -> no header injection -> no XSS on dashboard
"""
from flask import Blueprint, request, make_response, jsonify

response_builder_bp = Blueprint("response_builder", __name__)


# vuln-code-snippet start chain_header_xss_safe
@response_builder_bp.route("/api/set-theme")
def set_theme():
    """Set user theme preference as a cookie."""
    theme = request.args.get("theme", "light")
    theme = theme.replace("\r", "").replace("\n", "")
    response = make_response(jsonify({"status": "ok", "theme": theme}))
    response.headers["Set-Cookie"] = "theme=" + theme + "; Path=/"  # vuln-code-snippet safe-line chain_header_xss_safe
    return response
# vuln-code-snippet end chain_header_xss_safe
