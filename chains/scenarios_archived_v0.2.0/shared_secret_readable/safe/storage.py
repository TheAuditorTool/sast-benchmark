"""Shared secret storage utilities -- SAFE variant.

Writes the HMAC secret to disk with mode 0o600, restricting access to the
owning process only. No other local user can read the file and obtain the
secret, preventing offline token forgery.

Chain broken: secret file is owner-only -> attacker cannot read key -> cannot forge tokens
"""
import os

SECRET_FILE = "/tmp/app_hmac.key"


def write_shared_secret(secret: bytes):
    """Persist the shared HMAC secret to disk."""
    with open(SECRET_FILE, "wb") as fh:
        fh.write(secret)
    os.chmod(SECRET_FILE, 0o600)
