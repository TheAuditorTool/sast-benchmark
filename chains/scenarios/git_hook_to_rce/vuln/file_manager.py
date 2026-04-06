"""File manager endpoint -- VULNERABLE variant.

Provides an endpoint for uploading configuration files into the project
repository directory. The upload destination is built by joining the
repository root with a user-supplied relative path. No path canonicalization
is performed, so a path like ../../.git/hooks/pre-commit escapes the intended
upload directory and writes directly to the git hooks directory.

Once a hook script is written, the next call to /repo/commit (or any other
git operation in git_ops.py) will execute it as the server process.

Chain: POST /files/upload with path=../../.git/hooks/pre-commit -> file saved
       to hook location -> POST /repo/commit -> git executes hook -> RCE
Individual findings: path traversal in file write (high)
Chain finding: path traversal + git hook execution = unauthenticated RCE
               (critical, CWE-78)
"""
import os
from flask import Blueprint, request, jsonify

files_bp = Blueprint("files", __name__)

UPLOAD_DIR = os.environ.get("UPLOAD_DIR", "/var/repos/project/config")


# vuln-code-snippet start chain_git_hook_rce_vuln
@files_bp.route("/files/upload", methods=["POST"])
def upload_file():
    """Upload a configuration file to the repository.

    Expects multipart/form-data with fields: 'path' (relative destination)
    and 'file' (the file content).
    """
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400
    if "path" not in request.form:
        return jsonify({"error": "path field required"}), 400

    rel_path = request.form["path"]
    upload_file_obj = request.files["file"]

    # No canonicalization -- allows ../ sequences to escape UPLOAD_DIR
    destination = os.path.join(UPLOAD_DIR, rel_path)

    os.makedirs(os.path.dirname(destination), exist_ok=True)
    upload_file_obj.save(destination)  # vuln-code-snippet vuln-line chain_git_hook_rce_vuln

    return jsonify({
        "status": "uploaded",
        "destination": destination,
    }), 201
# vuln-code-snippet end chain_git_hook_rce_vuln
