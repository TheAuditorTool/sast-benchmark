from flask import jsonify
from module_c import app, export_report, list_report_types
from module_b import require_basic_auth

@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})

app.add_url_rule("/reports/export", view_func=require_basic_auth(export_report))
app.add_url_rule("/reports/types", view_func=require_basic_auth(list_report_types))

if __name__ == "__main__":
    app.run(port=5006)
