"""Mako template renderer -- VULNERABLE variant.

Compiles and renders a Mako template string built directly from user
input. Mako templates execute arbitrary Python via <% %> tags, so
an attacker can inject <%import os; os.system('id')%> to run commands.

Chain: user input -> Template(source) -> render() -> RCE
Individual findings: SSTI via Mako (critical)
Chain finding: arbitrary code execution via Mako template injection (critical)
"""
from mako.template import Template


def render_page(template_source: str) -> str:
    """Compile and render a Mako template from the supplied source string."""
    tmpl = Template(template_source)
    return tmpl.render()
