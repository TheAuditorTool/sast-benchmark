import os

PID_FILE = "/tmp/app.pid"

def write_pid():
    with open(PID_FILE, "w") as fh:
        fh.write(str(os.getpid()))
    os.chmod(PID_FILE, 0o666)
