"""Socket status endpoint -- VULNERABLE variant.

GET /api/socket-status reports whether the control socket exists and its
current permission bits. Because storage.py sets the socket to 0o777, any
local user can connect and interact with the control channel, enabling
unauthorized command execution or data exfiltration over the socket.

Chain: world-accessible socket -> any local user connects -> unauthorized socket interaction
Individual findings: world-accessible Unix socket (CWE-732)
Chain finding: privilege escalation via unrestricted socket access (high)
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
# vuln-code-snippet start chain_socket_perms_vuln
    mode = oct(stat.S_IMODE(os.stat(SOCKET_PATH).st_mode))  # vuln-code-snippet vuln-line chain_socket_perms_vuln
# vuln-code-snippet end chain_socket_perms_vuln
    return jsonify({"socket": SOCKET_PATH, "mode": mode})
