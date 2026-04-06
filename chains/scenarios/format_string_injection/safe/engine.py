"""Format-string template engine -- SAFE variant.

Uses string.Template with safe_substitute, which only supports
$identifier and ${identifier} placeholders. Attribute traversal
syntax ({obj.attr}) is not recognised and is emitted literally,
preventing exfiltration or code execution.

Chain broken: safe_substitute does not traverse attributes -> no injection
"""
import string


def render_template(template: str, context: dict) -> str:
    """Apply safe_substitute to the template using the supplied context dict."""
    return string.Template(template).safe_substitute(context)
