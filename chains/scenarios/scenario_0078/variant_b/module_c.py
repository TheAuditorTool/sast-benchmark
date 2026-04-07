import os
from flask import Blueprint, request, jsonify
from werkzeug.utils import secure_filename

storage_bp = Blueprint("storage", __name__)

STORAGE_DIR = os.environ.get("STORAGE_DIR", "/var/app/uploads")

@storage_bp.route("/files/<path:name>", methods=["PUT"])
def store_file(name: str):
    filename = secure_filename(name)
    if not filename:
        return jsonify({"error": "Invalid filename"}), 400

    dest = os.path.join(STORAGE_DIR, filename)
    os.makedirs(STORAGE_DIR, exist_ok=True)

    data = request.get_data()
    with open(dest, "wb") as fh:
        fh.write(data)

    return jsonify({"status": "stored", "name": filename, "size": len(data)}), 201

@storage_bp.route("/files/<path:name>", methods=["DELETE"])
def delete_file(name: str):
    filename = secure_filename(name)
    dest = os.path.join(STORAGE_DIR, filename)
    if not os.path.exists(dest):
        return jsonify({"error": "File not found"}), 404
    os.unlink(dest)
    return jsonify({"status": "deleted"}), 200

@storage_bp.route("/files", methods=["GET"])
def list_files():
    os.makedirs(STORAGE_DIR, exist_ok=True)
    names = [f for f in os.listdir(STORAGE_DIR) if os.path.isfile(os.path.join(STORAGE_DIR, f))]
    return jsonify({"files": names, "count": len(names)}), 200
