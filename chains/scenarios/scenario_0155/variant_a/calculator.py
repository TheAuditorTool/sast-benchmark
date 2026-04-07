from flask import Blueprint, request, jsonify

calculator_bp = Blueprint("calculator", __name__)

# vuln-code-snippet start ChainScenario0155A
@calculator_bp.route("/api/calc")
def calculate():
    expr = request.args.get("expr", "0")
    result = eval(expr)  # vuln-code-snippet target-line ChainScenario0155A
    return jsonify({"result": result})
# vuln-code-snippet end ChainScenario0155A
