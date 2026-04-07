import os

SECRET_FILE = "/tmp/app_hmac.key"

def write_shared_secret(secret: bytes):
    with open(SECRET_FILE, "wb") as fh:
        fh.write(secret)
    os.chmod(SECRET_FILE, 0o644)
