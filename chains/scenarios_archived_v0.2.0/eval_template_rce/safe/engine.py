"""Custom eval-based template engine -- SAFE variant.

Replaces {{ expr }} blocks with the string representation of the
corresponding key looked up from the context dict. No eval is
performed; unknown keys are replaced with an empty string.

Chain broken: context key lookup replaces eval -> no code execution
"""
import re


_EXPR_RE = re.compile(r"\{\{(.*?)\}\}")


def render_template(template: str, context: dict) -> str:
    """Replace {{ key }} blocks with the matching value from context."""
    def _lookup(match):
        key = match.group(1).strip()
        return str(context.get(key, ""))
    return _EXPR_RE.sub(_lookup, template)
