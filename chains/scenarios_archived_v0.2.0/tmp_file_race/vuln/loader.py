"""Data processing loader -- VULNERABLE variant.

POST /api/process writes caller-supplied data to a predictable temp path,
processes it, then reads the result back. Because the path is predictable,
an attacker can win the race between path computation and open() by placing
a symlink, diverting the write to an arbitrary file on disk.

Chain: predictable temp path -> attacker symlink placed -> open() follows link -> arbitrary write
Individual findings: TOCTOU race on predictable path (CWE-732)
Chain finding: arbitrary file overwrite via temp-file race (critical)
"""
import os
from flask import Blueprint, request, jsonify
from storage import get_temp_path

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/process", methods=["POST"])
def process_data():
    """Write request data to a temp file and return a processing token."""
    payload = request.get_data(as_text=True)
    tmp_path = get_temp_path()
# vuln-code-snippet start chain_tmp_file_race_vuln
    with open(tmp_path, "w") as fh:
        fh.write(payload)  # vuln-code-snippet vuln-line chain_tmp_file_race_vuln
# vuln-code-snippet end chain_tmp_file_race_vuln
    size = os.path.getsize(tmp_path)
    os.unlink(tmp_path)
    return jsonify({"bytes_processed": size})
