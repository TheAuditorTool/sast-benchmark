"""Task scheduler -- SAFE variant.

Accepts a task name, cron schedule expression, and shell command. All three
fields are validated against an allowlist before being written to the crontab.
Task names must be alphanumeric with hyphens/underscores. Schedule fields must
match the standard five-field cron format (digits, *, /, ,). Commands must
reference only pre-registered task scripts by basename. Injected newlines,
semicolons, pipes, and other shell metacharacters are rejected.

Chain: POST /schedule with injected fields -> validate_*() rejects any input
       containing shell metacharacters -> 400 returned -> no cron file written
Individual findings: none -- all fields validated before write
Chain finding: none -- injection prevented at input validation step (CWE-78 mitigated)
"""
import os
import re
from flask import Blueprint, request, jsonify
from task_runner import TASK_REGISTRY

scheduler_bp = Blueprint("scheduler", __name__)

CRON_DIR = os.environ.get("CRON_DIR", "/etc/cron.d")

_TASK_NAME_RE = re.compile(r'^[A-Za-z0-9_-]{1,64}$')
_CRON_FIELD_RE = re.compile(r'^[\d*/,\-]{1,20}$')
_ALLOWED_COMMANDS = frozenset([
    "/usr/local/bin/generate_report",
    "/usr/local/bin/export_data",
    "/usr/local/bin/cleanup_temp",
])


def validate_task_name(name: str) -> bool:
    return bool(_TASK_NAME_RE.match(name))


def validate_schedule(schedule: str) -> bool:
    parts = schedule.split()
    if len(parts) != 5:
        return False
    return all(_CRON_FIELD_RE.match(p) for p in parts)


def validate_command(command: str) -> bool:
    return command in _ALLOWED_COMMANDS


def write_crontab_entry(task_name: str, schedule: str, command: str) -> str:
    """Write a crontab entry file for a scheduled task.

    All parameters are validated before this function is called.
    """
    cron_content = f"{schedule} root {command}\n"
    cron_file = os.path.join(CRON_DIR, f"task_{task_name}")
    with open(cron_file, "w") as fh:
        fh.write(cron_content)
    return cron_file


# vuln-code-snippet start chain_cron_inject_safe
@scheduler_bp.route("/schedule", methods=["POST"])
def schedule_task():
    """Create a new scheduled task.

    Expects JSON with 'task_name', 'schedule' (cron expression), and 'command'.
    All fields are validated against allowlists before being written.
    """
    body = request.get_json(silent=True)
    if not body:
        return jsonify({"error": "JSON body required"}), 400

    task_name = body.get("task_name", "")
    schedule = body.get("schedule", "")
    command = body.get("command", "")

    if not all([task_name, schedule, command]):
        return jsonify({"error": "task_name, schedule, and command are required"}), 400

    if not validate_task_name(str(task_name)):
        return jsonify({"error": "task_name contains invalid characters"}), 400
    if not validate_schedule(str(schedule)):
        return jsonify({"error": "schedule must be a valid 5-field cron expression"}), 400
    if not validate_command(str(command)):
        return jsonify({"error": "command not in allowed list"}), 400

    cron_file = write_crontab_entry(task_name, schedule, command)  # vuln-code-snippet safe-line chain_cron_inject_safe
    TASK_REGISTRY[str(task_name)] = str(command)

    return jsonify({
        "status": "scheduled",
        "task_name": task_name,
        "cron_file": cron_file,
    }), 201
# vuln-code-snippet end chain_cron_inject_safe
