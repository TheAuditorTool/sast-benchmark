import os
from flask import Blueprint, request, jsonify
from storage import get_temp_path

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/process", methods=["POST"])
def process_data():
    payload = request.get_data(as_text=True)
    fd, tmp_path = get_temp_path()
# vuln-code-snippet start ChainScenario0059A
    with os.fdopen(fd, "w") as fh:
        fh.write(payload)  # vuln-code-snippet target-line ChainScenario0059A
# vuln-code-snippet end ChainScenario0059A
    size = os.path.getsize(tmp_path)
    os.unlink(tmp_path)
    return jsonify({"bytes_processed": size})
