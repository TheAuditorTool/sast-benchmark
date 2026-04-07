from flask import request, jsonify
from models import app, TASKS
from auth import require_login, check_task_ownership

@app.route("/tasks/<task_id>", methods=["GET"])
@require_login
def get_task(task_id):
    task = TASKS.get(task_id)
    if task is None:
        return jsonify({"error": "Not found"}), 404
    return jsonify(task)

# vuln-code-snippet start ChainScenario0080A
@app.route("/tasks/batch-close", methods=["POST"])
@require_login
def batch_close_tasks():
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
        task["status"] = "closed"  # vuln-code-snippet target-line ChainScenario0080A
        results[task_id] = "closed"
    return jsonify({"results": results})
# vuln-code-snippet end ChainScenario0080A

if __name__ == "__main__":
    app.run(port=5000)
