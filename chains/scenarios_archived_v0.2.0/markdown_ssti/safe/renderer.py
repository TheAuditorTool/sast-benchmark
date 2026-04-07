"""Markdown renderer -- SAFE variant.

Converts the Markdown source directly to HTML without any Jinja2
preprocessing. There is no template rendering step on user-supplied
text, so injected Jinja2 expressions are treated as literal text.

Chain broken: no Jinja2 preprocessing -> source converted as plain text -> no SSTI
"""


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
    """Convert Markdown source to HTML without template preprocessing."""
    return _simple_markdown(source)
