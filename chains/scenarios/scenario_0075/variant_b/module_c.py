import os

LOG_DIR = "/tmp/app_logs"
LOG_FILE = os.path.join(LOG_DIR, "access.log")

def setup_log_dir():
    os.makedirs(LOG_DIR, exist_ok=True)
    os.chmod(LOG_DIR, 0o700)
