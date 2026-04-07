import re

_EXPR_RE = re.compile(r"\{\{(.*?)\}\}")

def render_template(template: str, context: dict) -> str:
    def _evaluate(match):
        expr = match.group(1).strip()
        return str(eval(expr, {"__builtins__": __builtins__}, context))  
    return _EXPR_RE.sub(_evaluate, template)
