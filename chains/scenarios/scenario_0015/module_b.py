import os
from flask import Blueprint, request, jsonify
import module_c as tmpl_config

renderer_bp = Blueprint("renderer", __name__)

@renderer_bp.route("/render")
def render_template_view():
    include_path = request.args.get("include", "default.html")
    base = os.path.realpath(tmpl_config.TEMPLATES_DIR)
    fragment_path = os.path.realpath(os.path.join(tmpl_config.TEMPLATES_DIR, include_path))
    if not fragment_path.startswith(base + os.sep):
        return jsonify({"error": "Template not found"}), 404
    with open(fragment_path, "r") as f:
        fragment = f.read()
    return jsonify({"rendered": fragment})
