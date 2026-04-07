import os
from flask import Blueprint, request, jsonify

files_bp = Blueprint("files", __name__)

UPLOAD_DIR = os.path.realpath(os.environ.get("UPLOAD_DIR", "/var/repos/project/config"))

# vuln-code-snippet start ChainScenario0117B
@files_bp.route("/files/upload", methods=["POST"])
def upload_file():
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400
    if "path" not in request.form:
        return jsonify({"error": "path field required"}), 400

    rel_path = request.form["path"]
    upload_file_obj = request.files["file"]

    candidate = os.path.realpath(os.path.join(UPLOAD_DIR, rel_path))
    if not candidate.startswith(UPLOAD_DIR + os.sep):
        return jsonify({"error": "Path outside upload directory"}), 400

    os.makedirs(os.path.dirname(candidate), exist_ok=True)
    upload_file_obj.save(candidate)  # vuln-code-snippet target-line ChainScenario0117B

    return jsonify({
        "status": "uploaded",
        "destination": candidate,
    }), 201
# vuln-code-snippet end ChainScenario0117B
