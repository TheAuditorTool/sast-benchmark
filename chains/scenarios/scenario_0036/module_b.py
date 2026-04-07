from flask import Flask, request, jsonify
from module_a import init_db
from module_c import execute_transfer

app = Flask(__name__)

@app.route("/transfer", methods=["POST"])
def transfer():
    data = request.get_json(silent=True) or {}
    from_owner = data.get("from")
    to_owner = data.get("to")
    amount = data.get("amount")

    if not from_owner or not to_owner or amount is None:
        return jsonify({"error": "from, to, and amount are required"}), 400

    try:
        amount = float(amount)
    except (TypeError, ValueError):
        return jsonify({"error": "amount must be a number"}), 400

    result, status = execute_transfer(from_owner, to_owner, amount)
    return jsonify(result), status

@app.route("/balance/<owner>", methods=["GET"])
def balance(owner):
    from accounts import get_balance
    bal = get_balance(owner)
    if bal is None:
        return jsonify({"error": "Account not found"}), 404
    return jsonify({"owner": owner, "balance": bal}), 200

if __name__ == "__main__":
    init_db()
    app.run(port=5000)
