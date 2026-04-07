"""Mako template renderer -- SAFE variant.

The page template is a static Mako string. User-supplied content is
passed as a context variable (${content | h}) and HTML-escaped by
Mako's built-in h filter, preventing both XSS and template injection.

Chain broken: user input is a context variable, not template source -> no RCE
"""
from mako.template import Template


STATIC_TEMPLATE = "Page content: ${content | h}"


def render_page(content: str) -> str:
    """Render user content into a static Mako template as an escaped variable."""
    tmpl = Template(STATIC_TEMPLATE)
    return tmpl.render(content=content)
