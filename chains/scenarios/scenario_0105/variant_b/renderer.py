import os
from flask import Blueprint, request, jsonify
import templates as tmpl_config

renderer_bp = Blueprint("renderer", __name__)

@renderer_bp.route("/render")
def render_template_view():
    include_path = request.args.get("include", "default.html")
    fragment_path = os.path.join(tmpl_config.TEMPLATES_DIR, include_path)
# vuln-code-snippet start ChainScenario0105B
    with open(fragment_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0105B
        fragment = f.read()
# vuln-code-snippet end ChainScenario0105B
    return jsonify({"rendered": fragment})
