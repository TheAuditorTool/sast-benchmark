from flask import Flask, request, jsonify
from module_b import init_db
from module_c import place_order

app = Flask(__name__)

@app.route("/order", methods=["POST"])
def order():
    data = request.get_json(silent=True) or {}
    product_id = data.get("product_id")
    quantity = data.get("quantity")
    user_id = data.get("user_id")

    if product_id is None or quantity is None or user_id is None:
        return jsonify({"error": "product_id, quantity, and user_id are required"}), 400

    try:
        quantity = int(quantity)
    except (TypeError, ValueError):
        return jsonify({"error": "quantity must be an integer"}), 400

    result, status = place_order(product_id, quantity, user_id)
    return jsonify(result), status

@app.route("/stock/<int:product_id>", methods=["GET"])
def stock(product_id):
    from inventory import get_stock
    s = get_stock(product_id)
    if s is None:
        return jsonify({"error": "Product not found"}), 404
    return jsonify({"product_id": product_id, "stock": s}), 200

if __name__ == "__main__":
    init_db()
    app.run(port=5000)
