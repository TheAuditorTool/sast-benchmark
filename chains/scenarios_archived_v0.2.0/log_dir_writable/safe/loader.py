"""Log reader endpoint -- SAFE variant.

GET /api/logs reads the access log from a directory created with 0o700.
Only the owning process can write to the directory, so the entries read
here are guaranteed to come from the application itself.

Chain broken: log dir is owner-only -> no external write access -> loader reads authentic entries
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
# vuln-code-snippet start chain_log_dir_writable_safe
    with open(LOG_FILE, "r") as fh:
        lines = fh.readlines()[-50:]  # vuln-code-snippet safe-line chain_log_dir_writable_safe
# vuln-code-snippet end chain_log_dir_writable_safe
    return jsonify({"lines": [l.rstrip("\n") for l in lines]})
