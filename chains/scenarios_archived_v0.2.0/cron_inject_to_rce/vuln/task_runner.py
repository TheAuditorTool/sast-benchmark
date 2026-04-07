"""Task execution module -- IDENTICAL between vuln/ and safe/ variants.

Provides an endpoint to manually trigger a scheduled task by name, and
a background function used by the cron job itself to execute tasks. The
task command stored in the crontab is passed to the shell for execution.
This is the RCE execution point in the chain: if scheduler.py wrote an
injected command into the crontab, it will be executed here.

Chain: scheduler.py writes injected crontab entry -> cron fires at interval ->
       execute_task() runs shell command including injected payload -> RCE
Individual findings: subprocess shell execution (medium, expected for task runner)
Chain finding: combined with cron injection, executes arbitrary attacker commands
               (critical, CWE-78)
"""
import subprocess
import os
from flask import Blueprint, jsonify, request

runner_bp = Blueprint("runner", __name__)

TASK_REGISTRY: dict[str, str] = {}


def execute_task(task_name: str) -> dict:
    """Execute a registered task by running its shell command.

    Returns a dict with returncode, stdout, and stderr.
    """
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
    """Manually trigger a scheduled task by name."""
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
    """List all registered task names."""
    return jsonify({"tasks": list(TASK_REGISTRY.keys())}), 200
