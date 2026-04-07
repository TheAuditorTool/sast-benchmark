"""string.Template engine -- SAFE variant.

Uses string.Template.safe_substitute with an explicit, narrowly-scoped
mapping that exposes only the application name. Unknown placeholders are
left as-is. Globals are never exposed to the substitution mapping.

Chain broken: only APP_NAME is substitutable -> no globals exposure -> no disclosure
"""
import string

_SAFE_CONTEXT = {"APP_NAME": "StringTemplateApp"}


def render_template(template_str: str) -> str:
    """Render a string.Template using safe_substitute with a restricted context."""
    tmpl = string.Template(template_str)
    return tmpl.safe_substitute(_SAFE_CONTEXT)
