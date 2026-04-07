from flask import jsonify
from module_d import app, admin_query, audit_summary
from module_b import require_auth, login
from module_c import rate_limit

@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})

app.add_url_rule("/auth/login", view_func=rate_limit(login), methods=["POST"])
app.add_url_rule("/admin/query", view_func=require_auth(admin_query))
app.add_url_rule("/admin/audit", view_func=require_auth(audit_summary))

if __name__ == "__main__":
    app.run(port=5010)
