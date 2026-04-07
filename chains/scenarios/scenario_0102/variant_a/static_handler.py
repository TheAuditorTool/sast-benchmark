import os
from flask import Blueprint, jsonify
import config

static_handler_bp = Blueprint("static_handler", __name__)

@static_handler_bp.route("/asset/<path:filepath>")
def serve_asset(filepath):
    base = os.path.realpath(config.PUBLIC_DIR)
    file_path = os.path.realpath(os.path.join(config.PUBLIC_DIR, filepath))
    if not file_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
    if not os.path.isfile(file_path):
        return jsonify({"error": "Not found"}), 404
# vuln-code-snippet start ChainScenario0102A
    with open(file_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0102A
        content = f.read()
# vuln-code-snippet end ChainScenario0102A
    return jsonify({"path": filepath, "content": content})
