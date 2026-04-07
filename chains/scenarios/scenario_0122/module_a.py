from flask import jsonify, request
from module_b import app
from module_c import require_auth

@app.route("/admin")
@require_auth
def admin():
    return jsonify({
        "user": request.session["user_id"],
        "role": request.session["role"],
        "admin_data": "internal-config-42",
    })

if __name__ == "__main__":
    app.run(port=5010)
