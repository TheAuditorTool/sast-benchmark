"""Flask views for format_string_injection scenario -- VULNERABLE variant.

POST /message accepts a JSON body with 'template' and 'username' fields.
The template is a user-controlled string applied with str.format_map,
allowing attribute traversal attacks.

Chain: POST /message -> engine.render_template(user_template) -> info disclosure / RCE
"""
from flask import Blueprint, request, jsonify
from engine import render_template

message_bp = Blueprint("message", __name__)


# vuln-code-snippet start chain_format_string_vuln
@message_bp.route("/message", methods=["POST"])
def message():
    """Render a user-supplied format-string template."""
    template = request.json.get("template", "Hello, {username}!")
    username = request.json.get("username", "guest")
    result = render_template(template, {"username": username})  # vuln-code-snippet vuln-line chain_format_string_vuln
    return jsonify({"result": result})
# vuln-code-snippet end chain_format_string_vuln
