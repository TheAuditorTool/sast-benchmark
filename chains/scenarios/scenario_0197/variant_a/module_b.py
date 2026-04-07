import os

UPLOAD_DIR = "/tmp/uploads"

def ensure_upload_dir():
    os.makedirs(UPLOAD_DIR, exist_ok=True)

def resolve_path(filename):
    safe_name = os.path.basename(filename)
    if not safe_name:
        return None
    return os.path.join(UPLOAD_DIR, safe_name)
