"""Plugin upload endpoint -- VULNERABLE variant.

Accepts file uploads for a plugin system. Validates MIME type from the
Content-Type header but does NOT check the file extension. An attacker
can upload a .py file containing arbitrary code by spoofing the
Content-Type header to application/zip (claiming it's a zipped plugin).

The plugin runner in serve.py dynamically imports uploaded .py files,
so any uploaded Python file will be executed.

Chain: spoofed MIME type -> .py file saved -> serve.py imports it -> RCE
"""
import os
from flask import Flask, request, jsonify
from werkzeug.utils import secure_filename
from config import UPLOAD_DIR, MAX_UPLOAD_SIZE, ALLOWED_MIME_TYPES

app = Flask(__name__)
app.config["MAX_CONTENT_LENGTH"] = MAX_UPLOAD_SIZE


# vuln-code-snippet start chain_upload_rce_vuln
@app.route("/api/plugins/upload", methods=["POST"])
def upload_plugin():
    """Accept a plugin file upload after validating its MIME type."""
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400

    file = request.files["file"]
    if file.filename == "":
        return jsonify({"error": "No file selected"}), 400

    # Only validates MIME type (Content-Type header) -- attacker-controlled
    # Does NOT validate file extension -- a .py file with spoofed
    # Content-Type: application/zip passes this check
    mime_type = file.content_type
    if mime_type not in ALLOWED_MIME_TYPES:
        return jsonify({"error": "File type not allowed"}), 415

    filename = secure_filename(file.filename)
    if not filename:
        return jsonify({"error": "Invalid filename"}), 400

    destination = os.path.join(UPLOAD_DIR, filename)
    file.save(destination)  # vuln-code-snippet vuln-line chain_upload_rce_vuln

    return jsonify({
        "message": "Plugin uploaded successfully",
        "filename": filename,
        "run_url": f"/api/plugins/run/{filename.rsplit('.', 1)[0]}",
    }), 201
# vuln-code-snippet end chain_upload_rce_vuln


if __name__ == "__main__":
    app.run(port=5006)
