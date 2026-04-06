"""Unix socket setup utilities -- VULNERABLE variant.

Creates a Unix domain socket with mode 0o777, making it accessible to
any local user. Any process on the host can connect to the socket and
send commands or read data that should be restricted to the application.

Chain: world-accessible socket created -> any local user connects -> privileged socket commands executed
Individual findings: insecure Unix socket permissions (CWE-732)
Chain finding: unauthorized access via world-accessible Unix socket (high)
"""
import os
import socket

SOCKET_PATH = "/tmp/app_control.sock"


def create_socket():
    """Create a Unix domain socket and apply permissions."""
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    if os.path.exists(SOCKET_PATH):
        os.unlink(SOCKET_PATH)
    sock.bind(SOCKET_PATH)
    os.chmod(SOCKET_PATH, 0o777)
    return sock
