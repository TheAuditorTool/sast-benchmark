"""Socket status endpoint -- SAFE variant.

GET /api/socket-status reports the control socket path and permission bits.
Because storage.py sets the socket to 0o700, only the owning user can
connect, eliminating the unauthorized access vector.

Chain broken: socket is owner-only -> only owning process can connect -> no unauthorized access
"""
import os
import stat
from flask import Blueprint, jsonify
from storage import SOCKET_PATH, create_socket

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/socket-status", methods=["GET"])
def socket_status():
    """Return the socket path and its permission mode."""
    sock = create_socket()
    sock.close()
# vuln-code-snippet start chain_socket_perms_safe
    mode = oct(stat.S_IMODE(os.stat(SOCKET_PATH).st_mode))  # vuln-code-snippet safe-line chain_socket_perms_safe
# vuln-code-snippet end chain_socket_perms_safe
    return jsonify({"socket": SOCKET_PATH, "mode": mode})
