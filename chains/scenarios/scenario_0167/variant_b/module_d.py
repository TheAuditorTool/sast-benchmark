from flask import Blueprint, request, jsonify
from module_c import render_greeting

greet_bp = Blueprint("greet", __name__)

@greet_bp.route("/greet", methods=["POST"])
def greet():
    name = request.json.get("name", "World")
    template_str = "Hello, " + name + "!"
    result = render_greeting(template_str, {})
    return jsonify({"message": result})
