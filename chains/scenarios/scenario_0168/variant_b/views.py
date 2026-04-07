from flask import Blueprint, make_response
from cache import handle_conditional, RESOURCE_BODY

views_bp = Blueprint("views", __name__)

# vuln-code-snippet start ChainScenario0168B
@views_bp.route("/resource")
def resource():
    resp = make_response(RESOURCE_BODY)
    resp.headers["Cache-Control"] = "public, max-age=300"
    return handle_conditional(resp)  # vuln-code-snippet target-line ChainScenario0168B
# vuln-code-snippet end ChainScenario0168B
