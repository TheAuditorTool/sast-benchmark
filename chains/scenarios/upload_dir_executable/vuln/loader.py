"""File upload endpoint -- VULNERABLE variant.

POST /api/upload saves the request body as a named file in the upload
directory. Because storage.py sets the directory to 0o777 (including
execute), an attacker can upload a script file and, combined with a
misconfigured web server, have it executed directly.

Chain: upload dir execute+write for all -> attacker uploads script -> loader saves it -> script executed
Individual findings: upload dir with execute permission (CWE-732)
Chain finding: stored file execution via executable upload directory (critical)
"""
import os
from flask import Blueprint, request, jsonify
from storage import UPLOAD_DIR, setup_upload_dir

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/upload", methods=["POST"])
def upload_file():
    """Save uploaded data to a file in the upload directory."""
    setup_upload_dir()
    filename = request.args.get("name", "upload.bin")
    safe_name = os.path.basename(filename)
    dest = os.path.join(UPLOAD_DIR, safe_name)
# vuln-code-snippet start chain_upload_dir_exec_vuln
    with open(dest, "wb") as fh:
        fh.write(request.get_data())  # vuln-code-snippet vuln-line chain_upload_dir_exec_vuln
# vuln-code-snippet end chain_upload_dir_exec_vuln
    return jsonify({"saved": dest})
