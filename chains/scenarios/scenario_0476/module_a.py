from flask import jsonify
from module_b import app, restart_service, service_status
from module_c import require_session

@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})

app.add_url_rule("/ops/restart", view_func=require_session(restart_service), methods=["POST"])
app.add_url_rule("/ops/status", view_func=require_session(service_status))

if __name__ == "__main__":
    app.run(port=5008)
