"""Log viewer blueprint -- SAFE variant.

GET /logs?name=<filename> strips all directory components from the
supplied name using os.path.basename before joining with the log
directory, preventing traversal out of the log directory.

Chain: broken -- basename strips directory components, path stays within LOG_DIR
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, request, jsonify
import config

log_viewer_bp = Blueprint("log_viewer", __name__)


@log_viewer_bp.route("/logs")
def view_log():
    """Return the contents of a log file.

    FIXED: os.path.basename strips any directory traversal components so the
    path can only ever refer to a file directly inside LOG_DIR.
    """
    log_name = request.args.get("name", "app.log")
    safe_name = os.path.basename(log_name)
    if not safe_name:
        return jsonify({"error": "Invalid log name"}), 400
    log_path = os.path.join(config.LOG_DIR, safe_name)
# vuln-code-snippet start chain_log_path_safe
    with open(log_path, "r") as f:  # vuln-code-snippet safe-line chain_log_path_safe
        content = f.read()
# vuln-code-snippet end chain_log_path_safe
    return jsonify({"name": safe_name, "content": content})
