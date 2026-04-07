from flask import jsonify
from routes import app, search_users, list_users
from auth import require_auth

@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})

app.add_url_rule("/admin/users/search", view_func=require_auth(search_users))
app.add_url_rule("/admin/users", view_func=require_auth(list_users))

if __name__ == "__main__":
    app.run(port=5000)
