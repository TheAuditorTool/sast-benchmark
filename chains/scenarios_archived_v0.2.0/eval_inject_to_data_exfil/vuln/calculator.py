"""Expression calculator -- VULNERABLE variant.

GET /api/calc evaluates the user-supplied 'expr' query parameter with
Python's built-in eval(). An attacker can submit expressions that access
__import__('os').environ, read data_store.APP_CONFIG, or call arbitrary
functions, causing sensitive data to be returned in the response.

Chain: eval() on user input -> injected code reads env vars / APP_CONFIG
Individual findings: code injection via eval (critical)
Chain finding: sensitive data exfiltration via eval injection (critical)
"""
from flask import Blueprint, request, jsonify

calculator_bp = Blueprint("calculator", __name__)


# vuln-code-snippet start chain_eval_exfil_vuln
@calculator_bp.route("/api/calc")
def calculate():
    """Evaluate an arithmetic expression and return the result."""
    expr = request.args.get("expr", "0")
    result = eval(expr)  # vuln-code-snippet vuln-line chain_eval_exfil_vuln
    return jsonify({"result": result})
# vuln-code-snippet end chain_eval_exfil_vuln
