from flask import Blueprint, request, make_response, jsonify

response_builder_bp = Blueprint("response_builder", __name__)

# vuln-code-snippet start ChainScenario0234A
@response_builder_bp.route("/api/set-theme")
def set_theme():
    theme = request.args.get("theme", "light")
    response = make_response(jsonify({"status": "ok", "theme": theme}))
    response.headers["Set-Cookie"] = "theme=" + theme + "; Path=/"  # vuln-code-snippet target-line ChainScenario0234A
    return response
# vuln-code-snippet end ChainScenario0234A
