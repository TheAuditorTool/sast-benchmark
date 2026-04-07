import os
from flask import Flask, request, jsonify
from werkzeug.utils import secure_filename
from config import UPLOAD_DIR, MAX_UPLOAD_SIZE, ALLOWED_MIME_TYPES

app = Flask(__name__)
app.config["MAX_CONTENT_LENGTH"] = MAX_UPLOAD_SIZE

# vuln-code-snippet start ChainScenario0045B
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

    destination = os.path.join(UPLOAD_DIR, filename)
    file.save(destination)  # vuln-code-snippet target-line ChainScenario0045B

    return jsonify({
        "message": "Plugin uploaded successfully",
        "filename": filename,
        "run_url": f"/api/plugins/run/{filename.rsplit('.', 1)[0]}",
    }), 201
# vuln-code-snippet end ChainScenario0045B

if __name__ == "__main__":
    app.run(port=5006)
