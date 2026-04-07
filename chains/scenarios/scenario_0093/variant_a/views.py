from flask import Blueprint, request, make_response
from response_builder import build_link_header

views_bp = Blueprint("views", __name__)

# vuln-code-snippet start ChainScenario0093A
@views_bp.route("/page")
def page():
    resource = request.args.get("res", "/static/app.js")
    resp = make_response("<html><body>Page</body></html>")
    resp.headers["Link"] = build_link_header(resource)  # vuln-code-snippet target-line ChainScenario0093A
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp
# vuln-code-snippet end ChainScenario0093A
