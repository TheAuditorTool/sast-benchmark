from flask import jsonify
from deploy import app, trigger_deploy, deploy_status
from oauth import require_deploy_scope

@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})

app.add_url_rule("/deploy/run", view_func=require_deploy_scope(trigger_deploy), methods=["POST"])
app.add_url_rule("/deploy/status", view_func=require_deploy_scope(deploy_status))

if __name__ == "__main__":
    app.run(port=5003)
