"""File upload endpoint -- VULNERABLE variant.

Accepts administrator file uploads and saves them to the path specified in
the 'dest' form field. The destination is only checked to ensure it is an
absolute path; it is NOT restricted to a safe data directory. An attacker
(or compromised admin account) can set dest=/var/app/config/plugin_config.py
to overwrite the configuration file that config_loader.py exec()'s.

Chain: POST /admin/upload with dest=/var/app/config/plugin_config.py and
       malicious .py content -> file saved -> POST /admin/apply-config ->
       exec() runs attacker code -> RCE
Individual findings: unrestricted file write path (high)
Chain finding: combined with config exec, enables code execution (critical, CWE-94)
"""
import os
from flask import Blueprint, request, jsonify
from werkzeug.utils import secure_filename

upload_bp = Blueprint("upload", __name__)

DATA_DIR = os.environ.get("DATA_DIR", "/var/app/data")


# vuln-code-snippet start chain_config_overwrite_vuln
@upload_bp.route("/admin/upload", methods=["POST"])
def upload_file():
    """Upload a file to an administrator-specified destination path.

    Expects multipart/form-data with 'dest' (absolute destination path)
    and 'file' fields.
    """
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400
    if "dest" not in request.form:
        return jsonify({"error": "dest field required"}), 400

    dest = request.form["dest"]
    upload_file_obj = request.files["file"]

    if not os.path.isabs(dest):
        return jsonify({"error": "dest must be an absolute path"}), 400

    os.makedirs(os.path.dirname(dest), exist_ok=True)
    upload_file_obj.save(dest)  # vuln-code-snippet vuln-line chain_config_overwrite_vuln

    return jsonify({
        "status": "uploaded",
        "destination": dest,
    }), 201
# vuln-code-snippet end chain_config_overwrite_vuln
