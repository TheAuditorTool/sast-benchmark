"""File upload endpoint -- SAFE variant.

Accepts administrator file uploads and saves them within the DATA_DIR
directory only. The destination filename is extracted from the uploaded
file's original name (sanitized via secure_filename) and joined to DATA_DIR.
An attacker cannot specify an arbitrary destination path, so the config
directory remains unwritable via this endpoint.

Chain: POST /admin/upload -> destination forced to DATA_DIR/filename ->
       config directory not writable -> config file unchanged ->
       apply-config exec()'s original safe file -> no RCE
Individual findings: none -- destination restricted to DATA_DIR
Chain finding: none -- config overwrite path closed (CWE-94 mitigated)
"""
import os
from flask import Blueprint, request, jsonify
from werkzeug.utils import secure_filename

upload_bp = Blueprint("upload", __name__)

DATA_DIR = os.path.realpath(os.environ.get("DATA_DIR", "/var/app/data"))


# vuln-code-snippet start chain_config_overwrite_safe
@upload_bp.route("/admin/upload", methods=["POST"])
def upload_file():
    """Upload a file to the data directory.

    Expects multipart/form-data with a 'file' field.
    The destination is always within DATA_DIR; callers cannot specify
    an arbitrary destination path.
    """
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400

    upload_file_obj = request.files["file"]
    filename = secure_filename(upload_file_obj.filename or "upload")
    if not filename:
        return jsonify({"error": "Invalid filename"}), 400

    destination = os.path.join(DATA_DIR, filename)
    os.makedirs(DATA_DIR, exist_ok=True)
    upload_file_obj.save(destination)  # vuln-code-snippet safe-line chain_config_overwrite_safe

    return jsonify({
        "status": "uploaded",
        "destination": destination,
    }), 201
# vuln-code-snippet end chain_config_overwrite_safe
