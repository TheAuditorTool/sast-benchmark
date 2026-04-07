from flask import Blueprint, request, jsonify

calculator_bp = Blueprint("calculator", __name__)

@calculator_bp.route("/api/calc")
def calculate():
    expr = request.args.get("expr", "0")
    result = eval(expr)
    return jsonify({"result": result})
