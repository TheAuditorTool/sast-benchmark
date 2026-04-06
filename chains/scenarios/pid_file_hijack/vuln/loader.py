"""PID-based reload endpoint -- VULNERABLE variant.

POST /api/reload reads the PID file and sends SIGHUP to that process to
trigger a configuration reload. Because storage.py writes the PID file with
0o666, an attacker can replace the PID with an arbitrary value, causing the
application to signal an unintended process.

Chain: world-writable PID file -> attacker injects arbitrary PID -> SIGHUP sent to victim process
Individual findings: trust of world-writable PID file (CWE-732)
Chain finding: process signal injection via PID file hijack (high)
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
# vuln-code-snippet start chain_pid_file_vuln
    with open(PID_FILE, "r") as fh:
        pid = int(fh.read().strip())  # vuln-code-snippet vuln-line chain_pid_file_vuln
# vuln-code-snippet end chain_pid_file_vuln
    os.kill(pid, signal.SIGHUP)
    return jsonify({"signaled": pid})
