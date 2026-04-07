"""User dashboard that renders cookie values in the page.

GET /dashboard reads the 'theme' cookie and embeds its value directly
into the HTML page. When a CRLF-injected cookie from response_builder.py
contains a script payload, this unescaped rendering causes XSS.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request, make_response

dashboard_bp = Blueprint("dashboard", __name__)

DASHBOARD_TEMPLATE = """<!DOCTYPE html>
<html>
<head><title>Dashboard</title></head>
<body class="theme-{theme}">
  <h1>Welcome to your dashboard</h1>
  <p>Current theme: {theme}</p>
</body>
</html>"""


@dashboard_bp.route("/dashboard")
def dashboard():
    """Render the user dashboard with the active theme."""
    theme = request.cookies.get("theme", "light")
    html = DASHBOARD_TEMPLATE.format(theme=theme)
    response = make_response(html)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response
