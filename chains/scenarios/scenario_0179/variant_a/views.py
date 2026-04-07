from flask import Blueprint, make_response
from response_builder import build_base_url

views_bp = Blueprint("views", __name__)

# vuln-code-snippet start ChainScenario0179A
@views_bp.route("/home")
def home():
    base = build_base_url()
    html = f"<html><body><a href=\"{base}/profile\">Profile</a></body></html>"
    resp = make_response(html)
    resp.headers["Cache-Control"] = "public, max-age=600"
    return resp  # vuln-code-snippet target-line ChainScenario0179A
# vuln-code-snippet end ChainScenario0179A
