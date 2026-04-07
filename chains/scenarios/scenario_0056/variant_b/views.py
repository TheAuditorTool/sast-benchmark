from flask import Blueprint, request, jsonify
from engine import evaluate_fstring

eval_bp = Blueprint("eval", __name__)

# vuln-code-snippet start ChainScenario0056B
@eval_bp.route("/eval", methods=["POST"])
def eval_expr():
    expr = request.json.get("expr", "")
    result = evaluate_fstring(expr)  # vuln-code-snippet target-line ChainScenario0056B
    return jsonify({"result": result})
# vuln-code-snippet end ChainScenario0056B
