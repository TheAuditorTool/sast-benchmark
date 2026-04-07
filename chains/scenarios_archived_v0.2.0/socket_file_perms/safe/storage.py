"""Unix socket setup utilities -- SAFE variant.

Creates a Unix domain socket with mode 0o700, restricting access to the
owning user only. Other local users cannot connect to the socket,
preventing unauthorized command execution over the control channel.

Chain broken: socket is owner-only -> only owning process can connect -> no unauthorized access
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
    os.chmod(SOCKET_PATH, 0o700)
    return sock
