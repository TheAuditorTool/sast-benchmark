"""File storage module -- VULNERABLE variant.

Accepts file uploads and stores them in the per-user STORAGE_DIR. The write
path does not check whether an existing entry at the destination is a symlink.
An attacker can exploit a TOCTOU race: upload a legitimate file, then quickly
replace it with a symlink pointing to /etc/passwd before the read endpoint is
called. Alternatively, if the server process has write access to the storage
directory, the attacker creates the symlink directly via another request.

When reader.py subsequently opens the path, open() follows the symlink to the
sensitive target file.

Chain: PUT /files/<name> does not detect symlinks -> attacker replaces file
       with symlink -> GET /files/<name> follows symlink -> sensitive file read
Individual findings: no symlink detection before write (medium)
Chain finding: symlink in storage + reader following symlinks = arbitrary file
               read (high, CWE-22)
"""
import os
from flask import Blueprint, request, jsonify
from werkzeug.utils import secure_filename

storage_bp = Blueprint("storage", __name__)

STORAGE_DIR = os.environ.get("STORAGE_DIR", "/var/app/uploads")


# vuln-code-snippet start chain_symlink_read_vuln
@storage_bp.route("/files/<path:name>", methods=["PUT"])
def store_file(name: str):
    """Store an uploaded file in the storage directory.

    Does NOT check whether the destination path is currently a symlink,
    allowing a TOCTOU window where an attacker swaps the file for a symlink.
    """
    filename = secure_filename(name)
    if not filename:
        return jsonify({"error": "Invalid filename"}), 400

    dest = os.path.join(STORAGE_DIR, filename)
    os.makedirs(STORAGE_DIR, exist_ok=True)

    data = request.get_data()
    with open(dest, "wb") as fh:  # vuln-code-snippet vuln-line chain_symlink_read_vuln
        fh.write(data)

    return jsonify({"status": "stored", "name": filename, "size": len(data)}), 201
# vuln-code-snippet end chain_symlink_read_vuln


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
