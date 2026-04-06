"""Jinja2 template renderer -- SAFE variant.

The greeting template is a static string with a named placeholder.
User input is passed as a context variable, never embedded in the
template source, so Jinja2 syntax in the input is treated as data
and cannot execute arbitrary expressions.

Chain broken: user input is a context value, not template source -> no SSTI
"""
import jinja2


def render_greeting(name: str, context: dict) -> str:
    """Render a greeting with the user-supplied name as a safe context value."""
    env = jinja2.Environment(autoescape=True)
    template = env.from_string("Hello, {{ name }}!")
    return template.render(name=name, **context)
