from flask import Blueprint, make_response
from response_builder import build_lang_response

views_bp = Blueprint("views", __name__)

# vuln-code-snippet start ChainScenario0143A
@views_bp.route("/lang")
def lang():
    body = build_lang_response()
    resp = make_response(body)
    resp.headers["Cache-Control"] = "public, max-age=300"
    return resp  # vuln-code-snippet target-line ChainScenario0143A
# vuln-code-snippet end ChainScenario0143A
