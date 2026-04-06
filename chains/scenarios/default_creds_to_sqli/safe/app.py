"""Flask application entry point for the user management service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import jsonify
from routes import app, search_users, delete_user
from auth import require_auth, login


@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})


app.add_url_rule("/auth/login", view_func=login, methods=["POST"])
app.add_url_rule("/admin/users/search", view_func=require_auth(search_users))
app.add_url_rule("/admin/users/delete", view_func=require_auth(delete_user), methods=["POST"])


if __name__ == "__main__":
    app.run(port=5004)
