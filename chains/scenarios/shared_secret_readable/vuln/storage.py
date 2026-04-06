"""Shared secret storage utilities -- VULNERABLE variant.

Writes a shared HMAC secret to disk with mode 0o644, making it world-readable.
Any local user can read the file and obtain the secret, enabling them to forge
HMAC tokens that the loader accepts as authentic.

Chain: world-readable secret file -> any user reads key -> forged HMAC tokens accepted
Individual findings: secret stored with insecure permissions (CWE-732)
Chain finding: authentication bypass via stolen HMAC secret (critical)
"""
import os

SECRET_FILE = "/tmp/app_hmac.key"


def write_shared_secret(secret: bytes):
    """Persist the shared HMAC secret to disk."""
    with open(SECRET_FILE, "wb") as fh:
        fh.write(secret)
    os.chmod(SECRET_FILE, 0o644)
