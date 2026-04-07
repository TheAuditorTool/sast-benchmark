import jinja2

def render_page(content: str, context: dict) -> str:
    env = jinja2.Environment()
    template = env.from_string(content)
    return template.render(**context)
