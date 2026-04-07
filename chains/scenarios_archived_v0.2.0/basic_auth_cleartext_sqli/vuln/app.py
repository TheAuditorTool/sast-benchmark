"""Flask application entry point for the report export service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import jsonify
from routes import app, export_report, list_report_types
from middleware import require_basic_auth


@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})


app.add_url_rule("/reports/export", view_func=require_basic_auth(export_report))
app.add_url_rule("/reports/types", view_func=require_basic_auth(list_report_types))


if __name__ == "__main__":
    app.run(port=5006)
