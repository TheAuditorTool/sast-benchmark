"""Flask views for fstring_code_exec scenario -- SAFE variant.

POST /eval passes the client-supplied expression to engine.evaluate_fstring,
which returns it as literal text. No compilation or evaluation occurs.

Chain broken: evaluate_fstring returns literal -> no compile/eval -> no RCE
"""
from flask import Blueprint, request, jsonify
from engine import evaluate_fstring

eval_bp = Blueprint("eval", __name__)


# vuln-code-snippet start chain_fstring_exec_safe
@eval_bp.route("/eval", methods=["POST"])
def eval_expr():
    """Return the user-supplied expression as plain text."""
    expr = request.json.get("expr", "")
    result = evaluate_fstring(expr)  # vuln-code-snippet safe-line chain_fstring_exec_safe
    return jsonify({"result": result})
# vuln-code-snippet end chain_fstring_exec_safe
