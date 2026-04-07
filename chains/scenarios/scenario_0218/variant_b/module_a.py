import functools
from flask import request, jsonify
from module_c import app
from module_b import STRIPE_API_KEY

def _api_key_required(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-Api-Key") != STRIPE_API_KEY:
            return jsonify({"error": "Invalid API key"}), 401
        return f(*args, **kwargs)
    return decorated

@app.route("/crash")
def crash():
    return str(1 // 0)

@app.route("/billing/charge", methods=["POST"])
@_api_key_required
def charge():
    amount = request.get_json(force=True).get("amount", 0)
    return jsonify({"charged": amount})

if __name__ == "__main__":
    app.run(port=5000)
