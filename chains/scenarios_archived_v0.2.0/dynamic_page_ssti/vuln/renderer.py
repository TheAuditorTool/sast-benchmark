"""CMS page renderer -- VULNERABLE variant.

Treats the CMS-stored page content as a Jinja2 template source and
renders it directly. If an attacker can write content to the CMS
(e.g. via a separate content-editing endpoint), they can embed Jinja2
expressions to achieve SSTI leading to RCE.

Chain: CMS content -> from_string(content) -> render() -> RCE
Individual findings: SSTI via user-controlled CMS content (critical)
Chain finding: arbitrary code execution via CMS template injection (critical)
"""
import jinja2


def render_page(content: str, context: dict) -> str:
    """Render CMS page content as a Jinja2 template."""
    env = jinja2.Environment()
    template = env.from_string(content)
    return template.render(**context)
