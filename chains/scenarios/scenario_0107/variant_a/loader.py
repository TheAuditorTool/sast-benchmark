import os
import stat
from flask import Blueprint, jsonify
from storage import SOCKET_PATH, create_socket

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/socket-status", methods=["GET"])
def socket_status():
    sock = create_socket()
    sock.close()
# vuln-code-snippet start ChainScenario0107A
    mode = oct(stat.S_IMODE(os.stat(SOCKET_PATH).st_mode))  # vuln-code-snippet target-line ChainScenario0107A
# vuln-code-snippet end ChainScenario0107A
    return jsonify({"socket": SOCKET_PATH, "mode": mode})
