import jinja2

_NOTIFY_TEMPLATE = "Notification for {{ user }}: {{ message }}"

def render_notification(message: str, user: str) -> str:
    env = jinja2.Environment(autoescape=True)
    template = env.from_string(_NOTIFY_TEMPLATE)
    return template.render(message=message, user=user)
