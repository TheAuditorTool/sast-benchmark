import jinja2

def render_greeting(name: str, context: dict) -> str:
    env = jinja2.Environment(autoescape=True)
    template = env.from_string("Hello, {{ name }}!")
    return template.render(name=name, **context)
