"""string.Template engine -- VULNERABLE variant.

Uses string.Template.substitute (not safe_substitute) with a mapping
that exposes the full module globals. An attacker can supply a
$__class__ or ${__init__.__globals__[os]} style reference to access
arbitrary objects from the substitution mapping.

Chain: user template -> Template(template).substitute(globals()) -> info disclosure
Individual findings: unsafe Template.substitute with globals exposure (high)
Chain finding: sensitive data exfiltration via string.Template attribute traversal (high)
"""
import string


def render_template(template_str: str) -> str:
    """Render a string.Template using substitute with the module globals as mapping."""
    tmpl = string.Template(template_str)
    return tmpl.substitute(globals())
