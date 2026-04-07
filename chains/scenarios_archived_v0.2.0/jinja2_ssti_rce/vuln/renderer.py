"""Jinja2 template renderer -- VULNERABLE variant.

Accepts a raw template string and renders it directly via
jinja2.Environment.from_string, allowing server-side template injection.
An attacker can supply a name like {{ ''.__class__.__mro__[1].__subclasses__() }}
to enumerate classes, or use config.__class__.__init__.__globals__ to reach
os.system and execute arbitrary commands.

Chain: user input -> from_string -> render -> RCE
Individual findings: SSTI (critical)
Chain finding: arbitrary code execution via Jinja2 SSTI (critical)
"""
import jinja2


def render_greeting(template_str: str, context: dict) -> str:
    """Render a Jinja2 template string with the supplied context."""
    env = jinja2.Environment()
    template = env.from_string(template_str)
    return template.render(**context)
