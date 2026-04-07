from flask import jsonify
from module_c import app, run_analytics_query, dashboard_summary
from module_b import require_auth

@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})

app.add_url_rule("/analytics/query", view_func=require_auth(run_analytics_query))
app.add_url_rule("/analytics/dashboard", view_func=require_auth(dashboard_summary))

if __name__ == "__main__":
    app.run(port=5007)
