import os
import socket

SOCKET_PATH = "/tmp/app_control.sock"

def create_socket():
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    if os.path.exists(SOCKET_PATH):
        os.unlink(SOCKET_PATH)
    sock.bind(SOCKET_PATH)
    os.chmod(SOCKET_PATH, 0o777)
    return sock
