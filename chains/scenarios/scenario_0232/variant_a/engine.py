import jinja2

def render_config(config_str: str, context: dict) -> str:
    env = jinja2.Environment()
    template = env.from_string(config_str)
    return template.render(**context)
