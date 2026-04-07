from flask import jsonify, request
from auth import app
from session import require_auth

@app.route("/account")
@require_auth
def account():
    return jsonify({
        "user": request.session["user_id"],
        "role": request.session["role"],
    })

if __name__ == "__main__":
    app.run(port=5003)
