from flask import Blueprint, request, jsonify
from module_c import evaluate_fstring

eval_bp = Blueprint("eval", __name__)

@eval_bp.route("/eval", methods=["POST"])
def eval_expr():
    expr = request.json.get("expr", "")
    result = evaluate_fstring(expr)
    return jsonify({"result": result})
