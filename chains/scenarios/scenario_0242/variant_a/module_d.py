from flask import Blueprint, request, jsonify
from module_c import render_template

message_bp = Blueprint("message", __name__)

@message_bp.route("/message", methods=["POST"])
def message():
    template = request.json.get("template", "Hello, {username}!")
    username = request.json.get("username", "guest")
    result = render_template(template, {"username": username})
    return jsonify({"result": result})
