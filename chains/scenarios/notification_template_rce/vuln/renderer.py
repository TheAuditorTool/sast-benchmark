"""Notification renderer -- VULNERABLE variant.

Builds a Jinja2 template string by concatenating the user-supplied
notification message into the template source. An attacker can embed
Jinja2 expressions in the message to reach global builtins or execute
OS commands.

Chain: user message -> "Notification: " + message -> from_string -> render -> RCE
Individual findings: SSTI via notification message (critical)
Chain finding: arbitrary code execution via Jinja2 SSTI in notification (critical)
"""
import jinja2


def render_notification(message: str, user: str) -> str:
    """Render a notification by embedding the message in a Jinja2 template string."""
    env = jinja2.Environment()
    template_str = "Notification for " + user + ": " + message
    template = env.from_string(template_str)
    return template.render()
