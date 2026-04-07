import functools
from flask import request, jsonify
from config import app, USERS

def _get_user_by_token():
    token = request.headers.get("X-Api-Token", "")
    for uid, u in USERS.items():
        if u["api_token"] == token:
            return uid, u
    return None, None

def token_required(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        uid, user = _get_user_by_token()
        if uid is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user_id = uid
        request.current_user = user
        return f(*args, **kwargs)
    return decorated

@app.route("/api/account/delete", methods=["DELETE"])
@token_required
def delete_account():
    uid = request.current_user_id
    del USERS[uid]
    return jsonify({"status": "deleted"})

if __name__ == "__main__":
    app.run(port=5000)
