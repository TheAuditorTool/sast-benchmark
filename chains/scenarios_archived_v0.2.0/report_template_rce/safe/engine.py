"""Report generation engine -- SAFE variant.

Builds the report text with plain string concatenation. No Python
expression is constructed or evaluated from user-supplied data.

Chain broken: str concatenation replaces eval -> no code execution
"""


def generate_report(title: str, body: str) -> str:
    """Generate a report by concatenating the title and body as plain strings."""
    return "Report: " + title + "\n" + body
