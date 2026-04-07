import os
from flask import Blueprint, request, jsonify
from module_c import TASK_REGISTRY

scheduler_bp = Blueprint("scheduler", __name__)

CRON_DIR = os.environ.get("CRON_DIR", "/etc/cron.d")

def write_crontab_entry(task_name: str, schedule: str, command: str) -> str:
    cron_content = f"{schedule} root {command}\n"
    cron_file = os.path.join(CRON_DIR, f"task_{task_name}")
    with open(cron_file, "w") as fh:
        fh.write(cron_content)
    return cron_file

@scheduler_bp.route("/schedule", methods=["POST"])
def schedule_task():
    body = request.get_json(silent=True)
    if not body:
        return jsonify({"error": "JSON body required"}), 400

    task_name = body.get("task_name", "")
    schedule = body.get("schedule", "")
    command = body.get("command", "")

    if not all([task_name, schedule, command]):
        return jsonify({"error": "task_name, schedule, and command are required"}), 400

    cron_file = write_crontab_entry(task_name, schedule, command)
    TASK_REGISTRY[str(task_name)] = str(command)

    return jsonify({
        "status": "scheduled",
        "task_name": task_name,
        "cron_file": cron_file,
    }), 201
