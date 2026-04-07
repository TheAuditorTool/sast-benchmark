import os
from flask import Blueprint, request, jsonify
from werkzeug.utils import secure_filename

upload_bp = Blueprint("upload", __name__)

DATA_DIR = os.environ.get("DATA_DIR", "/var/app/data")

@upload_bp.route("/admin/upload", methods=["POST"])
def upload_file():
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400
    if "dest" not in request.form:
        return jsonify({"error": "dest field required"}), 400

    dest = request.form["dest"]
    upload_file_obj = request.files["file"]

    if not os.path.isabs(dest):
        return jsonify({"error": "dest must be an absolute path"}), 400

    os.makedirs(os.path.dirname(dest), exist_ok=True)
    upload_file_obj.save(dest)

    return jsonify({
        "status": "uploaded",
        "destination": dest,
    }), 201
