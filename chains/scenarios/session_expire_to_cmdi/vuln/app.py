"""Flask application entry point for the server diagnostics service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import jsonify
from diagnostics import app, run_ping, server_status
from session import require_session


@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})


app.add_url_rule("/diagnostics/ping", view_func=require_session(run_ping))
app.add_url_rule("/diagnostics/status", view_func=require_session(server_status))


if __name__ == "__main__":
    app.run(port=5001)
