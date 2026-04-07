import string

_SAFE_CONTEXT = {"APP_NAME": "StringTemplateApp"}

def render_template(template_str: str) -> str:
    tmpl = string.Template(template_str)
    return tmpl.safe_substitute(_SAFE_CONTEXT)
