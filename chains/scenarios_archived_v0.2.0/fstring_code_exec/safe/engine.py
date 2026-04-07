"""F-string execution engine -- SAFE variant.

Returns the user-supplied expression as plain text without compilation
or evaluation. No Python code in the input is executed.

Chain broken: no compile/eval -> user input returned as literal text -> no RCE
"""


def evaluate_fstring(user_expr: str) -> str:
    """Return the user expression as a plain string without evaluation."""
    return user_expr
