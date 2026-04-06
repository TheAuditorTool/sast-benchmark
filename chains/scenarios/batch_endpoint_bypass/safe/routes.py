"""Task management routes -- SAFE variant.

POST /tasks/batch-close now calls check_task_ownership() for each item
in the batch, rejecting tasks the caller does not own.

Chain: user submits task-104 -> per-item ownership check fails -> skipped with error
"""
from flask import request, jsonify
from models import app, TASKS
from auth import require_login, check_task_ownership


@app.route("/tasks/<task_id>", methods=["GET"])
@require_login
def get_task(task_id):
    """Retrieve a single task."""
    task = TASKS.get(task_id)
    if task is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(task)


# vuln-code-snippet start chain_batch_bypass_safe
@app.route("/tasks/batch-close", methods=["POST"])
@require_login
def batch_close_tasks():
    """Close multiple tasks. SAFE: per-item ownership verified for each task."""
    data = request.get_json(force=True) or {}
    task_ids = data.get("task_ids", [])
    if not isinstance(task_ids, list):
        return jsonify({"error": "task_ids must be a list"}), 400
    results = {}
    for task_id in task_ids:
        task = TASKS.get(task_id)
        if task is None:
            results[task_id] = "not_found"
            continue
        if not check_task_ownership(request.current_user_id, task_id):
            results[task_id] = "forbidden"
            continue
        task["status"] = "closed"  # vuln-code-snippet safe-line chain_batch_bypass_safe
        results[task_id] = "closed"
    return jsonify({"results": results})
# vuln-code-snippet end chain_batch_bypass_safe


if __name__ == "__main__":
    app.run(port=5000)
