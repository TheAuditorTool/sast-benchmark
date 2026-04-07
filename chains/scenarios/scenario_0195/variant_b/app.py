from flask import jsonify
from auth import app
from session import require_auth

@app.route("/profile")
@require_auth
def profile():
    return jsonify({
        "user_id": request.session["user_id"],
        "role":    request.session["role"],
    })

from flask import request  

if __name__ == "__main__":
    app.run(port=5001)
