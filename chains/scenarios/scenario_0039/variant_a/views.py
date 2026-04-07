from flask import Blueprint, request, jsonify
from response_builder import base_url_from_forwarded

views_bp = Blueprint("views", __name__)

# vuln-code-snippet start ChainScenario0039A
@views_bp.route("/reset-link")
def reset_link():
    token = request.args.get("token", "abc123")
    base = base_url_from_forwarded()
    link = f"{base}/reset?token={token}"
    from flask import make_response
    resp = make_response(jsonify({"link": link}))
    resp.headers["Cache-Control"] = "public, max-age=60"
    return resp  # vuln-code-snippet target-line ChainScenario0039A
# vuln-code-snippet end ChainScenario0039A
