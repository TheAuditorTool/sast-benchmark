"""Upload directory utilities -- SAFE variant.

Creates the upload directory with mode 0o755, granting execute and read to
others but not write. The owning process retains full control; other users
cannot add files. Without the world-write bit, attackers cannot place
malicious scripts in the directory.

Chain broken: upload dir not world-writable -> attacker cannot place scripts -> no execution path
"""
import os

UPLOAD_DIR = "/tmp/app_uploads"


def setup_upload_dir():
    """Create the upload directory with permissions."""
    os.makedirs(UPLOAD_DIR, exist_ok=True)
    os.chmod(UPLOAD_DIR, 0o755)
