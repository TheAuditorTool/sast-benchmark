"""F-string execution engine -- VULNERABLE variant.

Wraps user-supplied text in an f-string expression and compiles it
with compile() then eval(). Any Python expression that is valid inside
an f-string is executed, allowing __import__('os').system('id') style
payloads.

Chain: user input -> compile('f"' + input + '"') -> eval -> RCE
Individual findings: dynamic f-string compilation with user input (critical)
Chain finding: arbitrary code execution via f-string eval (critical)
"""


def evaluate_fstring(user_expr: str) -> str:
    """Compile and evaluate a user-supplied expression as an f-string."""
    code = compile('f"' + user_expr + '"', "<fstring>", "eval")
    return str(eval(code))  # noqa: S307
