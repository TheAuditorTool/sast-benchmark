"""PID file utilities -- VULNERABLE variant.

Writes the process PID to a file with mode 0o666, making it world-writable.
An attacker can overwrite the PID file to inject an arbitrary PID value that
administrative endpoints then act upon, enabling process signal injection.

Chain: world-writable PID file -> attacker overwrites with target PID -> loader sends signal to victim process
Individual findings: insecure PID file permissions (CWE-732)
Chain finding: signal injection via hijacked PID file (high)
"""
import os

PID_FILE = "/tmp/app.pid"


def write_pid():
    """Write the current process PID to the PID file."""
    with open(PID_FILE, "w") as fh:
        fh.write(str(os.getpid()))
    os.chmod(PID_FILE, 0o666)
