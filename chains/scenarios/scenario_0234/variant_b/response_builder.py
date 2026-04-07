from flask import Blueprint, request, make_response, jsonify

response_builder_bp = Blueprint("response_builder", __name__)

# vuln-code-snippet start ChainScenario0234B
@response_builder_bp.route("/api/set-theme")
def set_theme():
    theme = request.args.get("theme", "light")
    theme = theme.replace("\r", "").replace("\n", "")
    response = make_response(jsonify({"status": "ok", "theme": theme}))
    response.headers["Set-Cookie"] = "theme=" + theme + "; Path=/"  # vuln-code-snippet target-line ChainScenario0234B
    return response
# vuln-code-snippet end ChainScenario0234B
