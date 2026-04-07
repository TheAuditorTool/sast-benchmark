"""Flask views for jinja2_ssti_rce scenario -- SAFE variant.

POST /greet passes the user-supplied name as a context variable to
renderer.render_greeting. The template source is static, so injected
Jinja2 syntax in the name field is rendered as escaped text.

Chain broken: name is a context value -> render_greeting uses static template -> no RCE
"""
from flask import Blueprint, request, jsonify
from renderer import render_greeting

greet_bp = Blueprint("greet", __name__)


# vuln-code-snippet start chain_jinja2_ssti_safe
@greet_bp.route("/greet", methods=["POST"])
def greet():
    """Return a personalised greeting with the name passed as a safe context variable."""
    name = request.json.get("name", "World")
    result = render_greeting(name, {})  # vuln-code-snippet safe-line chain_jinja2_ssti_safe
    return jsonify({"message": result})
# vuln-code-snippet end chain_jinja2_ssti_safe
