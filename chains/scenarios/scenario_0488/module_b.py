import os
from flask import Blueprint, jsonify, request
from module_c import STORAGE_DIR

reader_bp = Blueprint("reader", __name__)

def resolve_path(name: str, base_dir: str) -> str | None:
    candidate = os.path.realpath(os.path.join(base_dir, name))
    if not candidate.startswith(os.path.realpath(base_dir) + os.sep):
        return None
    return candidate

@reader_bp.route("/files/<path:name>", methods=["GET"])
def read_file(name: str):
    path = resolve_path(name, STORAGE_DIR)
    if path is None:
        return jsonify({"error": "Path outside storage directory"}), 400
    if not os.path.exists(path):
        return jsonify({"error": "File not found"}), 404

    with open(path, "rb") as fh:
        data = fh.read()

    return data, 200, {"Content-Type": "application/octet-stream"}
