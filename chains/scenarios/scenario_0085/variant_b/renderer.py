import jinja2

_EMAIL_TEMPLATE = "Subject: {{ subject }}\nDear {{ recipient }},"

def render_email(subject: str, recipient: str) -> str:
    env = jinja2.Environment(autoescape=True)
    template = env.from_string(_EMAIL_TEMPLATE)
    return template.render(subject=subject, recipient=recipient)
