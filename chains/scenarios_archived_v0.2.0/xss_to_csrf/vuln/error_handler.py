"""Error page handler -- VULNERABLE variant.

GET /error reflects the user-supplied 'msg' query parameter directly
into the HTML error page with no escaping. An attacker can craft a
link with a script payload in 'msg' that executes in the victim's
browser and silently performs a CSRF action against admin_actions.py.

Chain: reflected XSS in error page -> injected script fires CSRF request
Individual findings: reflected XSS (high)
Chain finding: XSS-assisted CSRF against state-changing admin endpoint (critical)
"""
from flask import Blueprint, request, make_response

error_handler_bp = Blueprint("error_handler", __name__)

ERROR_TEMPLATE = """<!DOCTYPE html>
<html>
<head><title>Application Error</title></head>
<body>
  <h1>An error occurred</h1>
  <p class="error-detail">{message}</p>
  <a href="/">Return to home</a>
</body>
</html>"""


# vuln-code-snippet start chain_xss_csrf_vuln
@error_handler_bp.route("/error")
def show_error():
    """Render an error page with the user-supplied message."""
    message = request.args.get("msg", "Unknown error")
    html = ERROR_TEMPLATE.format(message=message)  # vuln-code-snippet vuln-line chain_xss_csrf_vuln
    response = make_response(html)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response
# vuln-code-snippet end chain_xss_csrf_vuln
