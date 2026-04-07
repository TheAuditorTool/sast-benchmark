import os
from flask import Blueprint, jsonify
from storage import LOG_FILE, setup_log_dir

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/logs", methods=["GET"])
def get_logs():
    setup_log_dir()
    if not os.path.exists(LOG_FILE):
        return jsonify({"lines": []})
# vuln-code-snippet start ChainScenario0075A
    with open(LOG_FILE, "r") as fh:
        lines = fh.readlines()[-50:]  # vuln-code-snippet target-line ChainScenario0075A
# vuln-code-snippet end ChainScenario0075A
    return jsonify({"lines": [l.rstrip("\n") for l in lines]})
