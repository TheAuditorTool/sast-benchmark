import os
from flask import Blueprint, jsonify
import config

static_handler_bp = Blueprint("static_handler", __name__)

@static_handler_bp.route("/asset/<path:filepath>")
def serve_asset(filepath):
    file_path = os.path.join(config.PROJECT_ROOT, filepath)
    if not os.path.isfile(file_path):
        return jsonify({"error": "Not found"}), 404
# vuln-code-snippet start ChainScenario0102B
    with open(file_path, "r") as f:  # vuln-code-snippet target-line ChainScenario0102B
        content = f.read()
# vuln-code-snippet end ChainScenario0102B
    return jsonify({"path": filepath, "content": content})
