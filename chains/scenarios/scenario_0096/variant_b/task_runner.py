import subprocess
import os
from flask import Blueprint, jsonify, request

runner_bp = Blueprint("runner", __name__)

TASK_REGISTRY: dict[str, str] = {}

def execute_task(task_name: str) -> dict:
    command = TASK_REGISTRY.get(task_name)
    if command is None:
        return {"error": f"Task '{task_name}' not registered"}
    result = subprocess.run(
        command,
        shell=True,
        capture_output=True,
        text=True,
        timeout=60,
    )
    return {
        "task": task_name,
        "returncode": result.returncode,
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
    }

@runner_bp.route("/tasks/run", methods=["POST"])
def run_task():
    body = request.get_json(silent=True) or {}
    task_name = body.get("task")
    if not task_name:
        return jsonify({"error": "task field required"}), 400
    result = execute_task(str(task_name))
    if "error" in result:
        return jsonify(result), 404
    return jsonify(result), 200

@runner_bp.route("/tasks/list", methods=["GET"])
def list_tasks():
    return jsonify({"tasks": list(TASK_REGISTRY.keys())}), 200
