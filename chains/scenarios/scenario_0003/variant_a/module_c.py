from module_a import app, admin_search_users, admin_dashboard
from module_b import require_auth

@app.route("/api/health")
def health():
    return {"status": "ok"}

app.add_url_rule("/admin/dashboard", view_func=require_auth(admin_dashboard))
app.add_url_rule("/admin/search", view_func=require_auth(admin_search_users))  

if __name__ == "__main__":
    app.run(port=5000)
