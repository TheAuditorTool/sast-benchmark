"""Email template renderer -- SAFE variant.

Subject and recipient are passed as context variables to a static
Jinja2 template with autoescape enabled. Injected Jinja2 syntax in
the subject is treated as escaped text and cannot execute.

Chain broken: subject is a context variable in a static template -> no SSTI
"""
import jinja2


_EMAIL_TEMPLATE = "Subject: {{ subject }}\nDear {{ recipient }},"


def render_email(subject: str, recipient: str) -> str:
    """Render an email body with subject and recipient as safe context values."""
    env = jinja2.Environment(autoescape=True)
    template = env.from_string(_EMAIL_TEMPLATE)
    return template.render(subject=subject, recipient=recipient)
