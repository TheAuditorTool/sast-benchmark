"""CI/CD pipeline routes -- SAFE variant.

PUT /pipelines/<id>/config now uses require_scope("write"), which rejects
read-only tokens before the config write is executed.

Chain: read-scoped token -> scope enforced server-side -> 403 (chain broken)
"""
from flask import request, jsonify
from models import app, PIPELINES
from auth import require_token, require_scope


@app.route("/pipelines/<pipe_id>", methods=["GET"])
@require_token
def get_pipeline(pipe_id):
    """Fetch pipeline details. Read scope sufficient."""
    pipeline = PIPELINES.get(pipe_id)
    if pipeline is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(pipeline)


# vuln-code-snippet start chain_token_scope_safe
@app.route("/pipelines/<pipe_id>/config", methods=["PUT"])
@require_scope("write")
def update_pipeline_config(pipe_id):
    """Update pipeline config. SAFE: write scope enforced server-side."""
    pipeline = PIPELINES.get(pipe_id)
    if pipeline is None:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    config = data.get("config", "")
    if not config:
        return jsonify({"error": "config field required"}), 400
    pipeline["config"] = config  # vuln-code-snippet safe-line chain_token_scope_safe
    return jsonify({"status": "updated", "pipeline": pipeline})
# vuln-code-snippet end chain_token_scope_safe


if __name__ == "__main__":
    app.run(port=5000)
