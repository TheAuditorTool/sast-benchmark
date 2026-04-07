from flask import jsonify, request
from auth import app
from session_store import require_auth

@app.route("/settings")
@require_auth
def settings():
    return jsonify({
        "user": request.session["user_id"],
        "role": request.session["role"],
    })

if __name__ == "__main__":
    app.run(port=5005)
