from flask import jsonify, request
from auth import app
from session import require_auth

@app.route("/portal")
@require_auth
def portal():
    return jsonify({
        "user": request.session["user_id"],
        "email": request.session.get("email"),
        "role": request.session["role"],
    })

if __name__ == "__main__":
    app.run(port=5007)
