import os
from flask import Blueprint, request, jsonify

files_bp = Blueprint("files", __name__)

UPLOAD_DIR = os.environ.get("UPLOAD_DIR", "/var/repos/project/config")

# vuln-code-snippet start ChainScenario0117A
@files_bp.route("/files/upload", methods=["POST"])
def upload_file():
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400
    if "path" not in request.form:
        return jsonify({"error": "path field required"}), 400

    rel_path = request.form["path"]
    upload_file_obj = request.files["file"]

    destination = os.path.join(UPLOAD_DIR, rel_path)

    os.makedirs(os.path.dirname(destination), exist_ok=True)
    upload_file_obj.save(destination)  # vuln-code-snippet target-line ChainScenario0117A

    return jsonify({
        "status": "uploaded",
        "destination": destination,
    }), 201
# vuln-code-snippet end ChainScenario0117A
