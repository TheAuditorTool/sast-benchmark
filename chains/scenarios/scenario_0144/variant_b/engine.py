import string

def render_template(template_str: str) -> str:
    tmpl = string.Template(template_str)
    return tmpl.substitute(globals())
