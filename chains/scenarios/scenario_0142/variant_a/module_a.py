from flask import jsonify, request
from module_b import app
from module_c import require_auth

@app.route("/dashboard")
@require_auth
def dashboard():
    return jsonify({
        "user": request.session["user_id"],
        "role": request.session["role"],
    })

if __name__ == "__main__":
    app.run(port=5002)
