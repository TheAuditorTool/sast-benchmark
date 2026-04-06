"""Task management routes -- VULNERABLE variant.

POST /tasks/batch-close accepts a list of task_ids and closes them all
without checking whether the caller owns each individual task.

Chain: user submits task-104 (admin's task) -> no per-item check -> status changed
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


# vuln-code-snippet start chain_batch_bypass_vuln
@app.route("/tasks/batch-close", methods=["POST"])
@require_login
def batch_close_tasks():
    """Close multiple tasks. VULNERABLE: per-item ownership check skipped."""
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
        task["status"] = "closed"  # vuln-code-snippet vuln-line chain_batch_bypass_vuln
        results[task_id] = "closed"
    return jsonify({"results": results})
# vuln-code-snippet end chain_batch_bypass_vuln


if __name__ == "__main__":
    app.run(port=5000)
