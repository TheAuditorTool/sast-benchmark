"""Widget renderer -- SAFE variant.

Only a whitelist of known template names are permitted. The template
source comes from a server-side registry, never from the client-supplied
config. Config values are passed as context variables in a static template.

Chain broken: template source is from server registry -> no user-controlled template -> no SSTI
"""
import jinja2

_ALLOWED_WIDGET_TEMPLATES = {
    "counter": "<div class='widget'>Count: {{ value }}</div>",
    "label": "<div class='widget'>{{ label }}</div>",
}

_DEFAULT_TEMPLATE = "<div class='widget'>{{ label }}</div>"


def render_widget(widget_config: dict) -> str:
    """Render a widget using a server-side template determined by the config type."""
    template_name = widget_config.get("type", "label")
    template_str = _ALLOWED_WIDGET_TEMPLATES.get(template_name, _DEFAULT_TEMPLATE)
    env = jinja2.Environment(autoescape=True)
    template = env.from_string(template_str)
    return template.render(**{k: str(v) for k, v in widget_config.items()})
