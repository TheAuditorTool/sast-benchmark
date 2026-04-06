"""Expression calculator -- SAFE variant.

GET /api/calc evaluates the user-supplied 'expr' query parameter with
ast.literal_eval(), which only allows Python literals (numbers, strings,
lists, dicts, tuples, booleans, None). Arbitrary expressions, function
calls, and attribute access are rejected, so no code injection is possible.

Chain broken: ast.literal_eval rejects arbitrary expressions -> no exfiltration
"""
import ast
from flask import Blueprint, request, jsonify

calculator_bp = Blueprint("calculator", __name__)


# vuln-code-snippet start chain_eval_exfil_safe
@calculator_bp.route("/api/calc")
def calculate():
    """Evaluate a literal expression and return the result."""
    expr = request.args.get("expr", "0")
    result = ast.literal_eval(expr)  # vuln-code-snippet safe-line chain_eval_exfil_safe
    return jsonify({"result": result})
# vuln-code-snippet end chain_eval_exfil_safe
