import jinja2

def render_greeting(template_str: str, context: dict) -> str:
    env = jinja2.Environment()
    template = env.from_string(template_str)
    return template.render(**context)
