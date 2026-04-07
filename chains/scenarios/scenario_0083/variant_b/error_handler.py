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

# vuln-code-snippet start ChainScenario0083B
@error_handler_bp.route("/error")
def show_error():
    message = request.args.get("msg", "Unknown error")
    safe_message = html.escape(message)
    html_page = ERROR_TEMPLATE.format(message=safe_message)  # vuln-code-snippet target-line ChainScenario0083B
    response = make_response(html_page)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response
# vuln-code-snippet end ChainScenario0083B
