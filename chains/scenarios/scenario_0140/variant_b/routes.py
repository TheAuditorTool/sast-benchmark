from flask import request, jsonify
from models import app, PIPELINES
from auth import require_token

@app.route("/pipelines/<pipe_id>", methods=["GET"])
@require_token
def get_pipeline(pipe_id):
    pipeline = PIPELINES.get(pipe_id)
    if pipeline is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(pipeline)

# vuln-code-snippet start ChainScenario0140B
@app.route("/pipelines/<pipe_id>/config", methods=["PUT"])
@require_token
def update_pipeline_config(pipe_id):
    pipeline = PIPELINES.get(pipe_id)
    if pipeline is None:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    config = data.get("config", "")
    if not config:
        return jsonify({"error": "config field required"}), 400
    pipeline["config"] = config  # vuln-code-snippet target-line ChainScenario0140B
    return jsonify({"status": "updated", "pipeline": pipeline})
# vuln-code-snippet end ChainScenario0140B

if __name__ == "__main__":
    app.run(port=5000)
