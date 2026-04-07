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
    theme = request.cookies.get("theme", "light")
    html = DASHBOARD_TEMPLATE.format(theme=theme)
    response = make_response(html)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response
