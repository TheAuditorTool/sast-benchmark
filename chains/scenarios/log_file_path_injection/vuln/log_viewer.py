"""Log viewer blueprint -- VULNERABLE variant.

GET /logs?name=<filename> opens a log file from the log directory using
the raw user-supplied filename.  An attacker can supply a name like
../../../etc/passwd or ../../app/config.py to read files outside the
log directory.

Chain: user-supplied log filename -> open() reads sensitive file
Individual findings: missing path restriction on log viewer (medium)
Chain finding: log path injection reads application secrets or /etc/shadow (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, request, jsonify
import config

log_viewer_bp = Blueprint("log_viewer", __name__)


@log_viewer_bp.route("/logs")
def view_log():
    """Return the contents of a log file.

    VULNERABLE: joins the user-supplied filename with LOG_DIR without
    stripping directory components, so traversal sequences escape the
    log directory.
    """
    log_name = request.args.get("name", "app.log")
    log_path = os.path.join(config.LOG_DIR, log_name)
# vuln-code-snippet start chain_log_path_vuln
    with open(log_path, "r") as f:  # vuln-code-snippet vuln-line chain_log_path_vuln
        content = f.read()
# vuln-code-snippet end chain_log_path_vuln
    return jsonify({"name": log_name, "content": content})
