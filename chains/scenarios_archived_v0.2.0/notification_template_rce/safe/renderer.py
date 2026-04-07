"""Notification renderer -- SAFE variant.

Message and user are passed as context variables to a static Jinja2
template with autoescape enabled. Jinja2 syntax in the message is
treated as escaped text and cannot execute.

Chain broken: message is context variable in static template -> no SSTI
"""
import jinja2


_NOTIFY_TEMPLATE = "Notification for {{ user }}: {{ message }}"


def render_notification(message: str, user: str) -> str:
    """Render a notification with message and user as safe context values."""
    env = jinja2.Environment(autoescape=True)
    template = env.from_string(_NOTIFY_TEMPLATE)
    return template.render(message=message, user=user)
