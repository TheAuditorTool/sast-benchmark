import os
from flask import Blueprint, request, jsonify
from module_c import get_temp_path

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/process", methods=["POST"])
def process_data():
    payload = request.get_data(as_text=True)
    tmp_path = get_temp_path()
    with open(tmp_path, "w") as fh:
        fh.write(payload)
    size = os.path.getsize(tmp_path)
    os.unlink(tmp_path)
    return jsonify({"bytes_processed": size})
