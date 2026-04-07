"""Error page handler -- SAFE variant.

GET /error reflects the user-supplied 'msg' query parameter into the
HTML error page, but applies html.escape() before insertion. Script
tags and HTML entities in the message are rendered as harmless text,
so the XSS vector that would enable CSRF is eliminated.

Chain broken: escaped message -> no XSS -> no CSRF
"""
import html
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


# vuln-code-snippet start chain_xss_csrf_safe
@error_handler_bp.route("/error")
def show_error():
    """Render an error page with the user-supplied message."""
    message = request.args.get("msg", "Unknown error")
    safe_message = html.escape(message)
    html_page = ERROR_TEMPLATE.format(message=safe_message)  # vuln-code-snippet safe-line chain_xss_csrf_safe
    response = make_response(html_page)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response
# vuln-code-snippet end chain_xss_csrf_safe
