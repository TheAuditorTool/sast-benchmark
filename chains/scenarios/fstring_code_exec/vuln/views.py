"""Flask views for fstring_code_exec scenario -- VULNERABLE variant.

POST /eval accepts a JSON body with an 'expr' field that is compiled
as an f-string expression and evaluated server-side, enabling RCE.

Chain: POST /eval -> engine.evaluate_fstring(expr) -> compile/eval -> RCE
"""
from flask import Blueprint, request, jsonify
from engine import evaluate_fstring

eval_bp = Blueprint("eval", __name__)


# vuln-code-snippet start chain_fstring_exec_vuln
@eval_bp.route("/eval", methods=["POST"])
def eval_expr():
    """Evaluate a user-supplied expression as a compiled f-string."""
    expr = request.json.get("expr", "")
    result = evaluate_fstring(expr)  # vuln-code-snippet vuln-line chain_fstring_exec_vuln
    return jsonify({"result": result})
# vuln-code-snippet end chain_fstring_exec_vuln
