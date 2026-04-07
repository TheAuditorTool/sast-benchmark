"""Data processing loader -- SAFE variant.

POST /api/process writes caller-supplied data to a temp file obtained via
tempfile.mkstemp, which atomically creates the file and returns an already-
open file descriptor. There is no window for a symlink race.

Chain broken: mkstemp gives atomic fd -> write through fd -> no symlink possible
"""
import os
from flask import Blueprint, request, jsonify
from storage import get_temp_path

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/process", methods=["POST"])
def process_data():
    """Write request data to a temp file and return a processing token."""
    payload = request.get_data(as_text=True)
    fd, tmp_path = get_temp_path()
# vuln-code-snippet start chain_tmp_file_race_safe
    with os.fdopen(fd, "w") as fh:
        fh.write(payload)  # vuln-code-snippet safe-line chain_tmp_file_race_safe
# vuln-code-snippet end chain_tmp_file_race_safe
    size = os.path.getsize(tmp_path)
    os.unlink(tmp_path)
    return jsonify({"bytes_processed": size})
