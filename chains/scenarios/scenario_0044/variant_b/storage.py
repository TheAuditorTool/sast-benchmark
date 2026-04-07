import os

STATIC_ROOT = "/var/app/static"
UPLOAD_DIR = "/var/app/static/uploads"
ENCRYPTION_KEY = "aes256-prod-key-do-not-leak"
INTERNAL_TOKEN = "internal-svc-token-xyz987"

def get_upload_path(filename):
    return os.path.join(UPLOAD_DIR, filename)
