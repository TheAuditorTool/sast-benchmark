"""Log directory setup utilities -- VULNERABLE variant.

Creates the application log directory with mode 0o777, allowing any
user to add, modify, or delete log files. An attacker with local access
can plant a malicious log entry or replace a log file with a symlink,
affecting downstream log consumers that trust log contents.

Chain: world-writable log dir created -> attacker injects log entry -> loader reads tampered log
Individual findings: insecure directory permissions (CWE-732)
Chain finding: log injection via world-writable log directory (high)
"""
import os

LOG_DIR = "/tmp/app_logs"
LOG_FILE = os.path.join(LOG_DIR, "access.log")


def setup_log_dir():
    """Create the log directory with permissions."""
    os.makedirs(LOG_DIR, exist_ok=True)
    os.chmod(LOG_DIR, 0o777)
