import string

def render_template(template: str, context: dict) -> str:
    return string.Template(template).safe_substitute(context)
