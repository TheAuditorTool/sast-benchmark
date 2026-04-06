"""Report generation engine -- VULNERABLE variant.

Builds a Python expression string from the user-supplied report title
and evaluates it via eval() to produce the final report text.
An attacker can embed __import__('os').system('id') inside the title
to execute arbitrary commands during report generation.

Chain: user title -> eval("'Report: ' + title") -> RCE
Individual findings: eval of user-controlled string (critical)
Chain finding: arbitrary code execution via report template eval (critical)
"""


def generate_report(title: str, body: str) -> str:
    """Generate a report by evaluating a constructed Python expression."""
    expression = "\"Report: \" + title + \"\\n\" + body"
    return eval(expression, {"title": title, "body": body})  # noqa: S307
