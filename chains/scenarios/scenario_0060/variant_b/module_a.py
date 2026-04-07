from flask import jsonify, request
from module_b import app
from module_c import require_auth

@app.route("/secure-data")
@require_auth
def secure_data():
    return jsonify({
        "user": request.session["user_id"],
        "secret": "sensitive-value-42",
    })

if __name__ == "__main__":
    app.run(port=5006)
