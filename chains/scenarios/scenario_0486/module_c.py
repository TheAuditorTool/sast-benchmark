import re
import jinja2

def _simple_markdown(text: str) -> str:
    lines = []
    for line in text.split("\n"):
        if line.startswith("# "):
            lines.append("<h1>" + line[2:] + "</h1>")
        elif line.strip():
            lines.append("<p>" + line + "</p>")
    return "\n".join(lines)

def render_markdown(source: str) -> str:
    env = jinja2.Environment()
    template = env.from_string(source)
    preprocessed = template.render()
    return _simple_markdown(preprocessed)
