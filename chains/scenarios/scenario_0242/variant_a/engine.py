def render_template(template: str, context: dict) -> str:
    return template.format_map(context)
