"""Format-string template engine -- VULNERABLE variant.

Calls str.format_map on a template string that is constructed from
user-supplied input. An attacker can inject format keys that traverse
object attributes, e.g. {secret.__class__} or {config.__init__.__globals__}
to exfiltrate sensitive values or reach code execution.

Chain: user input embedded in format string -> str.format_map(context) -> info disclosure / RCE
Individual findings: format string injection (high)
Chain finding: arbitrary attribute access via Python format-string injection (high)
"""


def render_template(template: str, context: dict) -> str:
    """Apply format_map to the template using the supplied context dict."""
    return template.format_map(context)
