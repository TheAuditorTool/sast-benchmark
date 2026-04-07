"""Log reader endpoint -- VULNERABLE variant.

GET /api/logs reads the most recent lines from the access log and returns
them. Because storage.py creates the log dir with 0o777, any local user
can write to that directory, injecting tampered entries that this endpoint
then reads and serves to callers.

Chain: world-writable log dir -> attacker injects entries -> loader reads and serves tampered data
Individual findings: log dir world-writable (CWE-732)
Chain finding: data tampering via log injection (high)
"""
import os
from flask import Blueprint, jsonify
from storage import LOG_FILE, setup_log_dir

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/logs", methods=["GET"])
def get_logs():
    """Return the last 50 lines of the access log."""
    setup_log_dir()
    if not os.path.exists(LOG_FILE):
        return jsonify({"lines": []})
# vuln-code-snippet start chain_log_dir_writable_vuln
    with open(LOG_FILE, "r") as fh:
        lines = fh.readlines()[-50:]  # vuln-code-snippet vuln-line chain_log_dir_writable_vuln
# vuln-code-snippet end chain_log_dir_writable_vuln
    return jsonify({"lines": [l.rstrip("\n") for l in lines]})
