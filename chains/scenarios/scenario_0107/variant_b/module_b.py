import os
import stat
from flask import Blueprint, jsonify
from module_c import SOCKET_PATH, create_socket

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/socket-status", methods=["GET"])
def socket_status():
    sock = create_socket()
    sock.close()
    mode = oct(stat.S_IMODE(os.stat(SOCKET_PATH).st_mode))
    return jsonify({"socket": SOCKET_PATH, "mode": mode})
