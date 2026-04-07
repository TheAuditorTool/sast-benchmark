from flask import Blueprint, request
from response_builder import build_response

views_bp = Blueprint("views", __name__)

_FILES = {
    "report": b"Annual report data",
    "logo": b"<binary image data>",
}

# vuln-code-snippet start ChainScenario0228A
@views_bp.route("/file")
def serve_file():
    name = request.args.get("name", "report")
    body = _FILES.get(name, b"Not found")
    return build_response(name, body)  # vuln-code-snippet target-line ChainScenario0228A
# vuln-code-snippet end ChainScenario0228A
