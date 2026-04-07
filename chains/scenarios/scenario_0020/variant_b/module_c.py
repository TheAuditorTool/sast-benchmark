import jinja2

def render_widget(widget_config: dict) -> str:
    env = jinja2.Environment()
    template_str = widget_config.get("template", "")
    template = env.from_string(template_str)
    return template.render(**widget_config)
