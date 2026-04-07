import os
import signal
from flask import Blueprint, jsonify
from module_c import PID_FILE, write_pid

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/reload", methods=["POST"])
def reload_config():
    write_pid()
    with open(PID_FILE, "r") as fh:
        pid = int(fh.read().strip())
    os.kill(pid, signal.SIGHUP)
    return jsonify({"signaled": pid})
