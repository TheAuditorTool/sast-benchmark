"""File viewer blueprint -- VULNERABLE variant.

GET /view?file=<name> checks that the supplied filename ends with .txt
to restrict access to text documents.  When the filename contains a null
byte (e.g. ../../etc/passwd%00.txt), the Python string ends-with check
passes because the string representation ends with '.txt', but some older
OS/libc implementations truncate at the null byte when opening the file.
The pattern is preserved here for benchmark completeness.

Chain: null-byte-suffixed filename bypasses extension check -> open() reads arbitrary file
Individual findings: missing null-byte sanitization (medium)
Chain finding: extension check bypass reads sensitive config files (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, request, jsonify
import config

viewer_bp = Blueprint("viewer", __name__)


@viewer_bp.route("/view")
def view_file():
    """Return text file contents.

    VULNERABLE: checks the extension but does not reject filenames containing
    null bytes, allowing the extension check to be bypassed via a trailing
    null byte followed by .txt.
    """
    filename = request.args.get("file", "")
    if not filename.endswith(".txt"):
        return jsonify({"error": "Only .txt files are allowed"}), 400
    file_path = os.path.join(config.VIEWER_BASE_DIR, filename)
# vuln-code-snippet start chain_null_byte_vuln
    with open(file_path, "r") as f:  # vuln-code-snippet vuln-line chain_null_byte_vuln
        content = f.read()
# vuln-code-snippet end chain_null_byte_vuln
    return jsonify({"file": filename, "content": content})
