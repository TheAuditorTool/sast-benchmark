"""Markdown renderer with Jinja2 preprocessing -- VULNERABLE variant.

Runs the user-supplied Markdown source through a Jinja2 render step
before converting it to HTML. An attacker can embed Jinja2 expressions
in the Markdown source to reach globals or execute OS commands before
the Markdown conversion occurs.

Chain: user markdown -> Jinja2 render (from_string) -> markdown() -> HTML with RCE
Individual findings: SSTI via Markdown preprocessing (critical)
Chain finding: arbitrary code execution via Jinja2 preprocessing of Markdown (critical)
"""
import re
import jinja2


def _simple_markdown(text: str) -> str:
    """Convert a subset of Markdown to HTML (headings and paragraphs)."""
    lines = []
    for line in text.split("\n"):
        if line.startswith("# "):
            lines.append("<h1>" + line[2:] + "</h1>")
        elif line.strip():
            lines.append("<p>" + line + "</p>")
    return "\n".join(lines)


def render_markdown(source: str) -> str:
    """Pre-process source as a Jinja2 template then convert to HTML."""
    env = jinja2.Environment()
    template = env.from_string(source)
    preprocessed = template.render()
    return _simple_markdown(preprocessed)
