"""Custom eval-based template engine -- VULNERABLE variant.

Parses {{ expr }} blocks in a template string and evaluates each
expression with eval(). An attacker can supply {{ __import__('os').system('id') }}
to execute arbitrary OS commands.

Chain: user input -> regex extract {{ expr }} -> eval(expr) -> RCE
Individual findings: use of eval with user input (critical)
Chain finding: arbitrary code execution via eval-based template engine (critical)
"""
import re


_EXPR_RE = re.compile(r"\{\{(.*?)\}\}")


def render_template(template: str, context: dict) -> str:
    """Render {{ expr }} blocks by evaluating each expression with eval."""
    def _evaluate(match):
        expr = match.group(1).strip()
        return str(eval(expr, {"__builtins__": __builtins__}, context))  # noqa: S307
    return _EXPR_RE.sub(_evaluate, template)
