import jinja2

_PAGE_TEMPLATE = "<html><body>{{ content }}</body></html>"

def render_page(content: str, context: dict) -> str:
    env = jinja2.Environment(autoescape=True)
    template = env.from_string(_PAGE_TEMPLATE)
    return template.render(content=content, **context)
