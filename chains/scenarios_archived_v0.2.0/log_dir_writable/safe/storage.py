"""Log directory setup utilities -- SAFE variant.

Creates the application log directory with mode 0o700, restricting access
to the owning process. No other local user can add or modify log files,
preventing log injection attacks.

Chain broken: log dir is owner-only -> no external write access -> loader reads authentic entries
"""
import os

LOG_DIR = "/tmp/app_logs"
LOG_FILE = os.path.join(LOG_DIR, "access.log")


def setup_log_dir():
    """Create the log directory with permissions."""
    os.makedirs(LOG_DIR, exist_ok=True)
    os.chmod(LOG_DIR, 0o700)
