from flask import Blueprint, make_response
from response_builder import build_url

views_bp = Blueprint("views", __name__)

# vuln-code-snippet start ChainScenario0201A
@views_bp.route("/asset")
def asset():
    canonical = build_url("/asset")
    html = f'<html><head><link rel="canonical" href="{canonical}"/></head><body>Asset</body></html>'
    resp = make_response(html)
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp  # vuln-code-snippet target-line ChainScenario0201A
# vuln-code-snippet end ChainScenario0201A
