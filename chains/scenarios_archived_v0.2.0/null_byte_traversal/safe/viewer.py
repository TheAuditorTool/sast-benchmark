"""File viewer blueprint -- SAFE variant.

GET /view?file=<name> rejects any filename containing a null byte before
performing the extension check, then canonicalizes the path to prevent
directory traversal.

Chain: broken -- null bytes rejected, path canonicalized before open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, request, jsonify
import config

viewer_bp = Blueprint("viewer", __name__)


@viewer_bp.route("/view")
def view_file():
    """Return text file contents.

    FIXED: filename is rejected immediately if it contains a null byte.
    Path is also canonicalized and constrained to the base directory.
    """
    filename = request.args.get("file", "")
    if "\x00" in filename:
        return jsonify({"error": "Invalid filename"}), 400
    if not filename.endswith(".txt"):
        return jsonify({"error": "Only .txt files are allowed"}), 400
    base = os.path.realpath(config.VIEWER_BASE_DIR)
    file_path = os.path.realpath(os.path.join(config.VIEWER_BASE_DIR, filename))
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
# vuln-code-snippet start chain_null_byte_safe
    with open(file_path, "r") as f:  # vuln-code-snippet safe-line chain_null_byte_safe
        content = f.read()
# vuln-code-snippet end chain_null_byte_safe
    return jsonify({"file": filename, "content": content})
