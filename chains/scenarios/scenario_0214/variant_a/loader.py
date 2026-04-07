import os
import signal
from flask import Blueprint, jsonify
from storage import PID_FILE, write_pid

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/reload", methods=["POST"])
def reload_config():
    write_pid()
# vuln-code-snippet start ChainScenario0214A
    with open(PID_FILE, "r") as fh:
        pid = int(fh.read().strip())  # vuln-code-snippet target-line ChainScenario0214A
# vuln-code-snippet end ChainScenario0214A
    os.kill(pid, signal.SIGHUP)
    return jsonify({"signaled": pid})
