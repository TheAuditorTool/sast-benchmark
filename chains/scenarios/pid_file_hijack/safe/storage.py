"""PID file utilities -- SAFE variant.

Writes the process PID to a file with mode 0o600, restricting write access
to the owning process. Other local users cannot tamper with the PID value,
preventing process signal injection.

Chain broken: PID file is owner-only -> attacker cannot overwrite -> loader signals correct process
"""
import os

PID_FILE = "/tmp/app.pid"


def write_pid():
    """Write the current process PID to the PID file."""
    with open(PID_FILE, "w") as fh:
        fh.write(str(os.getpid()))
    os.chmod(PID_FILE, 0o600)
