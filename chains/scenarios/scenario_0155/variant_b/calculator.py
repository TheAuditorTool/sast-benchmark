import ast
from flask import Blueprint, request, jsonify

calculator_bp = Blueprint("calculator", __name__)

# vuln-code-snippet start ChainScenario0155B
@calculator_bp.route("/api/calc")
def calculate():
    expr = request.args.get("expr", "0")
    result = ast.literal_eval(expr)  # vuln-code-snippet target-line ChainScenario0155B
    return jsonify({"result": result})
# vuln-code-snippet end ChainScenario0155B
