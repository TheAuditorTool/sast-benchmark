import os

UPLOAD_DIR = "/tmp/app_uploads"

def setup_upload_dir():
    os.makedirs(UPLOAD_DIR, exist_ok=True)
    os.chmod(UPLOAD_DIR, 0o777)
