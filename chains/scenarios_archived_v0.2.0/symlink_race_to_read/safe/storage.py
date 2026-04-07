"""File storage module -- SAFE variant.

Accepts file uploads and stores them in STORAGE_DIR. Before writing, checks
whether the destination path is a symlink using os.path.islink(). If a symlink
exists at the target location, the request is rejected. This closes the TOCTOU
window: an attacker who pre-creates a symlink at the destination cannot get
the server to overwrite it with file data, and the existing symlink is rejected
before reader.py can follow it.

Combined with O_NOFOLLOW semantics in reader.py (safe variant), symlinks in
the storage directory cannot be used to read sensitive files.

Chain: PUT /files/<name> detects symlink at destination -> 400 returned ->
       file not written over symlink -> reader.py finds no symlink to follow
Individual findings: none -- symlink checked before write
Chain finding: none -- symlink race condition closed at storage step (CWE-22 mitigated)
"""
import os
from flask import Blueprint, request, jsonify
from werkzeug.utils import secure_filename

storage_bp = Blueprint("storage", __name__)

STORAGE_DIR = os.environ.get("STORAGE_DIR", "/var/app/uploads")


# vuln-code-snippet start chain_symlink_read_safe
@storage_bp.route("/files/<path:name>", methods=["PUT"])
def store_file(name: str):
    """Store an uploaded file in the storage directory.

    Rejects the request if a symlink already exists at the target path,
    preventing symlink-race exploitation.
    """
    filename = secure_filename(name)
    if not filename:
        return jsonify({"error": "Invalid filename"}), 400

    dest = os.path.join(STORAGE_DIR, filename)
    os.makedirs(STORAGE_DIR, exist_ok=True)

    if os.path.islink(dest):
        return jsonify({"error": "Symlink at destination is not allowed"}), 400

    data = request.get_data()
    with open(dest, "wb") as fh:  # vuln-code-snippet safe-line chain_symlink_read_safe
        fh.write(data)

    return jsonify({"status": "stored", "name": filename, "size": len(data)}), 201
# vuln-code-snippet end chain_symlink_read_safe


@storage_bp.route("/files/<path:name>", methods=["DELETE"])
def delete_file(name: str):
    """Delete a stored file by name."""
    filename = secure_filename(name)
    dest = os.path.join(STORAGE_DIR, filename)
    if not os.path.exists(dest):
        return jsonify({"error": "File not found"}), 404
    os.unlink(dest)
    return jsonify({"status": "deleted"}), 200


@storage_bp.route("/files", methods=["GET"])
def list_files():
    """List all files in the storage directory."""
    os.makedirs(STORAGE_DIR, exist_ok=True)
    names = [f for f in os.listdir(STORAGE_DIR) if os.path.isfile(os.path.join(STORAGE_DIR, f))]
    return jsonify({"files": names, "count": len(names)}), 200
