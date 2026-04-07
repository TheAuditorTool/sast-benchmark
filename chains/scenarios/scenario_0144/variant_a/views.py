from flask import Blueprint, request, jsonify
from engine import render_template

template_bp = Blueprint("template", __name__)

# vuln-code-snippet start ChainScenario0144A
@template_bp.route("/template", methods=["POST"])
def apply_template():
    template_str = request.json.get("template", "$APP_NAME")
    result = render_template(template_str)  # vuln-code-snippet target-line ChainScenario0144A
    return jsonify({"result": result})
# vuln-code-snippet end ChainScenario0144A
