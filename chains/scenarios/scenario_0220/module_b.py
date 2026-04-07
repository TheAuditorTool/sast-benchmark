import os
from flask import Blueprint, request, jsonify
import module_c as tmpl_config

renderer_bp = Blueprint("renderer", __name__)

@renderer_bp.route("/render")
def render_template_view():
    include_path = request.args.get("include", "default.html")
    fragment_path = os.path.join(tmpl_config.TEMPLATES_DIR, include_path)
    with open(fragment_path, "r") as f:
        fragment = f.read()
    return jsonify({"rendered": fragment})
