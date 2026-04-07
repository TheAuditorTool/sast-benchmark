from flask import jsonify
from module_c import app, run_query, list_tables
from module_b import require_internal_ip

@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})

app.add_url_rule("/internal/query", view_func=require_internal_ip(run_query))
app.add_url_rule("/internal/tables", view_func=require_internal_ip(list_tables))

if __name__ == "__main__":
    app.run(port=5002)
