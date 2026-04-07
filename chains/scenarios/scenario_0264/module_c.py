import os
from flask import Flask, request, jsonify
from werkzeug.utils import secure_filename
from module_a import UPLOAD_DIR, MAX_UPLOAD_SIZE, ALLOWED_MIME_TYPES

app = Flask(__name__)
app.config["MAX_CONTENT_LENGTH"] = MAX_UPLOAD_SIZE

ALLOWED_EXTENSIONS = frozenset(["zip"])

@app.route("/api/plugins/upload", methods=["POST"])
def upload_plugin():
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400

    file = request.files["file"]
    if file.filename == "":
        return jsonify({"error": "No file selected"}), 400

    mime_type = file.content_type
    if mime_type not in ALLOWED_MIME_TYPES:
        return jsonify({"error": "File type not allowed"}), 415

    filename = secure_filename(file.filename)
    if not filename:
        return jsonify({"error": "Invalid filename"}), 400

    ext = filename.rsplit(".", 1)[-1].lower() if "." in filename else ""
    if ext not in ALLOWED_EXTENSIONS:
        return jsonify({"error": f"Extension '.{ext}' not allowed"}), 415

    destination = os.path.join(UPLOAD_DIR, filename)
    file.save(destination)

    return jsonify({
        "message": "Plugin uploaded successfully",
        "filename": filename,
    }), 201

if __name__ == "__main__":
    app.run(port=5006)
