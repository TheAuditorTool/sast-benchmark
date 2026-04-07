from flask import Blueprint, request
from response_builder import build_redirect

views_bp = Blueprint("views", __name__)

# vuln-code-snippet start ChainScenario0152B
@views_bp.route("/redirect")
def do_redirect():
    url = request.args.get("url", "/")
    return build_redirect(url)  # vuln-code-snippet target-line ChainScenario0152B
# vuln-code-snippet end ChainScenario0152B
