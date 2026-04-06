"""File upload endpoint -- SAFE variant.

POST /api/upload saves the request body as a named file in the upload
directory. Because storage.py sets the directory to 0o755, no local
attacker can place additional files there; only the application process
can write to the directory.

Chain broken: upload dir not world-writable -> attacker cannot place scripts -> no execution path
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
# vuln-code-snippet start chain_upload_dir_exec_safe
    with open(dest, "wb") as fh:
        fh.write(request.get_data())  # vuln-code-snippet safe-line chain_upload_dir_exec_safe
# vuln-code-snippet end chain_upload_dir_exec_safe
    return jsonify({"saved": dest})
