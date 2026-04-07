from flask import Blueprint, request, jsonify
from module_c import render_template

template_bp = Blueprint("template", __name__)

@template_bp.route("/template", methods=["POST"])
def apply_template():
    template_str = request.json.get("template", "$APP_NAME")
    result = render_template(template_str)
    return jsonify({"result": result})
