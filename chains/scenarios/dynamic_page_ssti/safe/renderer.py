"""CMS page renderer -- SAFE variant.

CMS content is HTML-escaped and inserted into a static Jinja2 template
as a variable. Template syntax in the content cannot be executed.

Chain broken: content is escaped and passed as context variable -> no SSTI
"""
import jinja2


_PAGE_TEMPLATE = "<html><body>{{ content }}</body></html>"


def render_page(content: str, context: dict) -> str:
    """Render CMS content escaped inside a static page template."""
    env = jinja2.Environment(autoescape=True)
    template = env.from_string(_PAGE_TEMPLATE)
    return template.render(content=content, **context)
