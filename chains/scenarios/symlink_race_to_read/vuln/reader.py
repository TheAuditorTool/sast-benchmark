"""File reader module -- IDENTICAL between vuln/ and safe/ variants.

Serves stored file contents by name. This is the read sink in the chain:
if the file at the requested path is a symlink (created by an attacker via
storage.py or a race condition), the open() call follows it to the symlink
target. The vulnerable storage.py does not detect symlinks before writing,
so an attacker can swap a legitimate file for a symlink to /etc/passwd after
upload and before the read endpoint is called.

Chain: storage.py stores file (or symlink survives) -> GET /files/<name>
       -> resolve_path() returns symlink path -> open() follows symlink ->
       /etc/passwd or other sensitive file contents returned
Individual findings: open() follows symlinks (medium, context-dependent)
Chain finding: combined with symlink creation in storage, enables arbitrary
               file read (high, CWE-22)
"""
import os
from flask import Blueprint, jsonify, request
from storage import STORAGE_DIR

reader_bp = Blueprint("reader", __name__)


def resolve_path(name: str, base_dir: str) -> str | None:
    """Resolve a filename to an absolute path within base_dir.

    Returns None if the resolved path is outside base_dir.
    """
    candidate = os.path.realpath(os.path.join(base_dir, name))
    if not candidate.startswith(os.path.realpath(base_dir) + os.sep):
        return None
    return candidate


@reader_bp.route("/files/<path:name>", methods=["GET"])
def read_file(name: str):
    """Return the contents of a stored file."""
    path = resolve_path(name, STORAGE_DIR)
    if path is None:
        return jsonify({"error": "Path outside storage directory"}), 400
    if not os.path.exists(path):
        return jsonify({"error": "File not found"}), 404

    with open(path, "rb") as fh:
        data = fh.read()

    return data, 200, {"Content-Type": "application/octet-stream"}
