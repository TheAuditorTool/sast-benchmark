from flask import Flask, request, jsonify
from coupons import init_db
from redemption import redeem_coupon

app = Flask(__name__)

@app.route("/redeem", methods=["POST"])
def redeem():
    data = request.get_json(silent=True) or {}
    code = data.get("code")
    user_id = data.get("user_id")

    if not code or user_id is None:
        return jsonify({"error": "code and user_id are required"}), 400

    result, status = redeem_coupon(code, user_id)
    return jsonify(result), status

@app.route("/coupon/<code>", methods=["GET"])
def coupon_info(code):
    from coupons import get_coupon
    coupon = get_coupon(code)
    if not coupon:
        return jsonify({"error": "Coupon not found"}), 404
    return jsonify(coupon), 200

if __name__ == "__main__":
    init_db()
    app.run(port=5000)
