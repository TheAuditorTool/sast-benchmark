from flask import jsonify
from module_b import app, run_ping, server_status
from module_c import require_session

@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})

app.add_url_rule("/diagnostics/ping", view_func=require_session(run_ping))
app.add_url_rule("/diagnostics/status", view_func=require_session(server_status))

if __name__ == "__main__":
    app.run(port=5001)
