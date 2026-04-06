"""PID-based reload endpoint -- SAFE variant.

POST /api/reload reads the PID file and sends SIGHUP. Because storage.py
writes the PID file with 0o600, only the owning process can modify it.
The PID read here is guaranteed to be the value written by this application.

Chain broken: PID file is owner-only -> attacker cannot overwrite -> loader signals correct process
"""
import os
import signal
from flask import Blueprint, jsonify
from storage import PID_FILE, write_pid

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/reload", methods=["POST"])
def reload_config():
    """Send SIGHUP to the PID recorded in the PID file."""
    write_pid()
# vuln-code-snippet start chain_pid_file_safe
    with open(PID_FILE, "r") as fh:
        pid = int(fh.read().strip())  # vuln-code-snippet safe-line chain_pid_file_safe
# vuln-code-snippet end chain_pid_file_safe
    os.kill(pid, signal.SIGHUP)
    return jsonify({"signaled": pid})
