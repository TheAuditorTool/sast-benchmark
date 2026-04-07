"""File manager endpoint -- SAFE variant.

Provides an endpoint for uploading configuration files into the project
repository directory. The destination path is canonicalized with os.path.realpath()
and verified to remain inside UPLOAD_DIR before the file is written. A path
like ../../.git/hooks/pre-commit is caught and rejected.

Chain: POST /files/upload with path=../../.git/hooks/pre-commit -> realpath
       resolves to outside UPLOAD_DIR -> 400 rejected -> hook never written
Individual findings: none -- path traversal prevented by canonicalization
Chain finding: none -- RCE chain broken at write step (CWE-78 mitigated)
"""
import os
from flask import Blueprint, request, jsonify

files_bp = Blueprint("files", __name__)

UPLOAD_DIR = os.path.realpath(os.environ.get("UPLOAD_DIR", "/var/repos/project/config"))


# vuln-code-snippet start chain_git_hook_rce_safe
@files_bp.route("/files/upload", methods=["POST"])
def upload_file():
    """Upload a configuration file to the repository.

    Expects multipart/form-data with fields: 'path' (relative destination)
    and 'file' (the file content).
    Path is canonicalized and confined to UPLOAD_DIR to prevent traversal.
    """
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400
    if "path" not in request.form:
        return jsonify({"error": "path field required"}), 400

    rel_path = request.form["path"]
    upload_file_obj = request.files["file"]

    candidate = os.path.realpath(os.path.join(UPLOAD_DIR, rel_path))
    if not candidate.startswith(UPLOAD_DIR + os.sep):
        return jsonify({"error": "Path outside upload directory"}), 400

    os.makedirs(os.path.dirname(candidate), exist_ok=True)
    upload_file_obj.save(candidate)  # vuln-code-snippet safe-line chain_git_hook_rce_safe

    return jsonify({
        "status": "uploaded",
        "destination": candidate,
    }), 201
# vuln-code-snippet end chain_git_hook_rce_safe
