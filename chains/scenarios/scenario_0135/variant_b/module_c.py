import jinja2

def render_notification(message: str, user: str) -> str:
    env = jinja2.Environment()
    template_str = "Notification for " + user + ": " + message
    template = env.from_string(template_str)
    return template.render()
