import jinja2

def render_email(subject: str, recipient: str) -> str:
    env = jinja2.Environment()
    template_str = "Subject: " + subject + "\nDear " + recipient + ","
    template = env.from_string(template_str)
    return template.render()
