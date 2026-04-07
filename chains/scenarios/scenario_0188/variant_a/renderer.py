from mako.template import Template

STATIC_TEMPLATE = "Page content: ${content | h}"

def render_page(content: str) -> str:
    tmpl = Template(STATIC_TEMPLATE)
    return tmpl.render(content=content)
