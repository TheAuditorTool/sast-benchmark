"""Flask application entry point for the internal data query service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import jsonify
from routes import app, run_query, list_tables
from middleware import require_internal_ip


@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})


app.add_url_rule("/internal/query", view_func=require_internal_ip(run_query))
app.add_url_rule("/internal/tables", view_func=require_internal_ip(list_tables))


if __name__ == "__main__":
    app.run(port=5002)
