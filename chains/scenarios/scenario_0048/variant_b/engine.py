import re

_EXPR_RE = re.compile(r"\{\{(.*?)\}\}")

def render_template(template: str, context: dict) -> str:
    def _lookup(match):
        key = match.group(1).strip()
        return str(context.get(key, ""))
    return _EXPR_RE.sub(_lookup, template)
