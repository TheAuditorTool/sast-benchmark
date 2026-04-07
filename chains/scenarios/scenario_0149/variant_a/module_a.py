from flask import jsonify, request
from module_b import app
from module_c import require_auth

@app.route("/orders")
@require_auth
def orders():
    return jsonify({
        "user": request.session["user_id"],
        "orders": ["order-1", "order-2"],
    })

if __name__ == "__main__":
    app.run(port=5012)
