from mako.template import Template

def render_page(template_source: str) -> str:
    tmpl = Template(template_source)
    return tmpl.render()
