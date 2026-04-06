"""CI/CD pipeline routes -- VULNERABLE variant.

PUT /pipelines/<id>/config requires only a valid token, not a write-scoped
token. A read-only token can update pipeline configuration.

Chain: read-scoped token -> scope not verified server-side -> config overwritten
"""
from flask import request, jsonify
from models import app, PIPELINES
from auth import require_token


@app.route("/pipelines/<pipe_id>", methods=["GET"])
@require_token
def get_pipeline(pipe_id):
    """Fetch pipeline details. Read scope sufficient."""
    pipeline = PIPELINES.get(pipe_id)
    if pipeline is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(pipeline)


# vuln-code-snippet start chain_token_scope_vuln
@app.route("/pipelines/<pipe_id>/config", methods=["PUT"])
@require_token
def update_pipeline_config(pipe_id):
    """Update pipeline config. VULNERABLE: write scope not enforced server-side."""
    pipeline = PIPELINES.get(pipe_id)
    if pipeline is None:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    config = data.get("config", "")
    if not config:
        return jsonify({"error": "config field required"}), 400
    pipeline["config"] = config  # vuln-code-snippet vuln-line chain_token_scope_vuln
    return jsonify({"status": "updated", "pipeline": pipeline})
# vuln-code-snippet end chain_token_scope_vuln


if __name__ == "__main__":
    app.run(port=5000)
