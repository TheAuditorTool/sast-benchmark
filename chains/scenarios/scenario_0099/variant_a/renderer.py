def _simple_markdown(text: str) -> str:
    lines = []
    for line in text.split("\n"):
        if line.startswith("# "):
            lines.append("<h1>" + line[2:] + "</h1>")
        elif line.strip():
            lines.append("<p>" + line + "</p>")
    return "\n".join(lines)

def render_markdown(source: str) -> str:
    return _simple_markdown(source)
