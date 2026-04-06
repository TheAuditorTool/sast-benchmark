"""Flask application for the file upload service.

This file is IDENTICAL between vuln/ and safe/ variants.
Route wiring only; the race condition is in upload.py.

Chain: concurrent POST /upload -> os.path.exists() check -> open(path, 'wb') write
Individual findings: TOCTOU on file existence check (medium)
Chain finding: concurrent upload overwrites another user's file (CWE-362, critical)
"""
from flask import Flask, request, jsonify
from storage import ensure_upload_dir
from upload import save_upload

app = Flask(__name__)

MAX_SIZE = 1 * 1024 * 1024  # 1 MB


@app.route("/upload", methods=["POST"])
def upload():
    filename = request.args.get("filename")
    if not filename:
        return jsonify({"error": "filename query parameter is required"}), 400

    data = request.get_data()
    if len(data) > MAX_SIZE:
        return jsonify({"error": "File too large"}), 413

    result, status = save_upload(filename, data)
    return jsonify(result), status


@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok"}), 200


if __name__ == "__main__":
    ensure_upload_dir()
    app.run(port=5000)
