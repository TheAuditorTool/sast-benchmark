from flask import jsonify, request
from module_b import app
from module_c import require_auth

@app.route("/api/data")
@require_auth
def data():
    return jsonify({
        "user": request.token_record["user_id"],
        "device": request.token_record["device_id"],
    })

if __name__ == "__main__":
    app.run(port=5008)
