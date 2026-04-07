import jinja2

_ALLOWED_WIDGET_TEMPLATES = {
    "counter": "<div class='widget'>Count: {{ value }}</div>",
    "label": "<div class='widget'>{{ label }}</div>",
}

_DEFAULT_TEMPLATE = "<div class='widget'>{{ label }}</div>"

def render_widget(widget_config: dict) -> str:
    template_name = widget_config.get("type", "label")
    template_str = _ALLOWED_WIDGET_TEMPLATES.get(template_name, _DEFAULT_TEMPLATE)
    env = jinja2.Environment(autoescape=True)
    template = env.from_string(template_str)
    return template.render(**{k: str(v) for k, v in widget_config.items()})
