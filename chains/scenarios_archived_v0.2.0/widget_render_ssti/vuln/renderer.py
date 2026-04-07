"""Widget renderer -- VULNERABLE variant.

Takes a widget configuration dict that contains a 'template' key and
renders it as a Jinja2 template. An attacker who controls the widget
config (e.g. via a dashboard customisation endpoint) can inject Jinja2
expressions to traverse globals and execute OS commands.

Chain: widget config -> from_string(config['template']) -> render() -> RCE
Individual findings: SSTI via widget config template (critical)
Chain finding: arbitrary code execution via widget template injection (critical)
"""
import jinja2


def render_widget(widget_config: dict) -> str:
    """Render a widget using its 'template' field as a Jinja2 template source."""
    env = jinja2.Environment()
    template_str = widget_config.get("template", "")
    template = env.from_string(template_str)
    return template.render(**widget_config)
