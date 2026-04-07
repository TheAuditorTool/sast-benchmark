"""Flask views for format_string_injection scenario -- SAFE variant.

POST /message passes the client-supplied template to engine.render_template,
which uses string.Template.safe_substitute. The $-based placeholder syntax
cannot traverse object attributes, breaking the injection chain.

Chain broken: safe_substitute -> no attribute traversal -> no info disclosure
"""
from flask import Blueprint, request, jsonify
from engine import render_template

message_bp = Blueprint("message", __name__)


# vuln-code-snippet start chain_format_string_safe
@message_bp.route("/message", methods=["POST"])
def message():
    """Render a format template using safe_substitute."""
    template = request.json.get("template", "Hello, $username!")
    username = request.json.get("username", "guest")
    result = render_template(template, {"username": username})  # vuln-code-snippet safe-line chain_format_string_safe
    return jsonify({"result": result})
# vuln-code-snippet end chain_format_string_safe
