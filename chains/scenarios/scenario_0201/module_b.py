import os
from flask import Blueprint, jsonify
from module_c import LOG_FILE, setup_log_dir

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/logs", methods=["GET"])
def get_logs():
    setup_log_dir()
    if not os.path.exists(LOG_FILE):
        return jsonify({"lines": []})
    with open(LOG_FILE, "r") as fh:
        lines = fh.readlines()[-50:]
    return jsonify({"lines": [l.rstrip("\n") for l in lines]})
