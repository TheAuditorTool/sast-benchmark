from flask import request, jsonify
from module_b import app, PIPELINES
from module_a import require_token, require_scope

@app.route("/pipelines/<pipe_id>", methods=["GET"])
@require_token
def get_pipeline(pipe_id):
    pipeline = PIPELINES.get(pipe_id)
    if pipeline is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(pipeline)

@app.route("/pipelines/<pipe_id>/config", methods=["PUT"])
@require_scope("write")
def update_pipeline_config(pipe_id):
    pipeline = PIPELINES.get(pipe_id)
    if pipeline is None:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    config = data.get("config", "")
    if not config:
        return jsonify({"error": "config field required"}), 400
    pipeline["config"] = config
    return jsonify({"status": "updated", "pipeline": pipeline})

if __name__ == "__main__":
    app.run(port=5000)
