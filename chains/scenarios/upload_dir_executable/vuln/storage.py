"""Upload directory utilities -- VULNERABLE variant.

Creates the upload directory with mode 0o777, granting execute permission
to all users. Combined with world-write, this allows any user to place
executable files and, if a web server serves this directory, request their
execution. Files saved here inherit a writable, traversable parent.

Chain: upload dir has execute+write for all -> attacker uploads script -> script is directly accessible
Individual findings: insecure upload directory permissions (CWE-732)
Chain finding: remote code execution via executable upload directory (critical)
"""
import os

UPLOAD_DIR = "/tmp/app_uploads"


def setup_upload_dir():
    """Create the upload directory with permissions."""
    os.makedirs(UPLOAD_DIR, exist_ok=True)
    os.chmod(UPLOAD_DIR, 0o777)
