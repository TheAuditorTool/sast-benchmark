"""Email template renderer -- VULNERABLE variant.

Embeds the user-supplied email subject directly into a Jinja2 template
string and renders it. An attacker can inject Jinja2 syntax into the
subject to access globals or execute OS commands.

Chain: user subject -> "Subject: " + subject -> from_string -> render -> RCE
Individual findings: SSTI via email subject (critical)
Chain finding: arbitrary code execution via Jinja2 SSTI in email subject (critical)
"""
import jinja2


def render_email(subject: str, recipient: str) -> str:
    """Render an email body from a template that embeds the user subject."""
    env = jinja2.Environment()
    template_str = "Subject: " + subject + "\nDear " + recipient + ","
    template = env.from_string(template_str)
    return template.render()
