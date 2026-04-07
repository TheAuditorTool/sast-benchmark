import os
from flask import Blueprint, request, jsonify
from werkzeug.utils import secure_filename

upload_bp = Blueprint("upload", __name__)

DATA_DIR = os.path.realpath(os.environ.get("DATA_DIR", "/var/app/data"))

@upload_bp.route("/admin/upload", methods=["POST"])
def upload_file():
    if "file" not in request.files:
        return jsonify({"error": "No file part in request"}), 400

    upload_file_obj = request.files["file"]
    filename = secure_filename(upload_file_obj.filename or "upload")
    if not filename:
        return jsonify({"error": "Invalid filename"}), 400

    destination = os.path.join(DATA_DIR, filename)
    os.makedirs(DATA_DIR, exist_ok=True)
    upload_file_obj.save(destination)

    return jsonify({
        "status": "uploaded",
        "destination": destination,
    }), 201
