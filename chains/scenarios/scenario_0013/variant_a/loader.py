import os
from flask import Blueprint, request, jsonify
from storage import UPLOAD_DIR, setup_upload_dir

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/upload", methods=["POST"])
def upload_file():
    setup_upload_dir()
    filename = request.args.get("name", "upload.bin")
    safe_name = os.path.basename(filename)
    dest = os.path.join(UPLOAD_DIR, safe_name)
# vuln-code-snippet start ChainScenario0013A
    with open(dest, "wb") as fh:
        fh.write(request.get_data())  # vuln-code-snippet target-line ChainScenario0013A
# vuln-code-snippet end ChainScenario0013A
    return jsonify({"saved": dest})
