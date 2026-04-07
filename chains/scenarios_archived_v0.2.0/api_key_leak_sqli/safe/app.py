"""Flask application entry point for the data export service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import jsonify
from routes import app, export_records, schema_info
from middleware import require_api_key


@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})


app.add_url_rule("/export/records", view_func=require_api_key(export_records))
app.add_url_rule("/export/schema", view_func=require_api_key(schema_info))


if __name__ == "__main__":
    app.run(port=5009)
