"""Flask views for jinja2_ssti_rce scenario -- VULNERABLE variant.

POST /greet accepts a JSON body with a 'name' field. The name is
interpolated directly into a Jinja2 template string that is then
rendered server-side. Because the template is constructed from
unvalidated input, an attacker can inject Jinja2 syntax to reach RCE.

Chain: POST /greet -> renderer.render_greeting(template_str) -> RCE
"""
from flask import Blueprint, request, jsonify
from renderer import render_greeting

greet_bp = Blueprint("greet", __name__)


# vuln-code-snippet start chain_jinja2_ssti_vuln
@greet_bp.route("/greet", methods=["POST"])
def greet():
    """Return a personalised greeting rendered from a Jinja2 template."""
    name = request.json.get("name", "World")
    template_str = "Hello, " + name + "!"
    result = render_greeting(template_str, {})  # vuln-code-snippet vuln-line chain_jinja2_ssti_vuln
    return jsonify({"message": result})
# vuln-code-snippet end chain_jinja2_ssti_vuln
