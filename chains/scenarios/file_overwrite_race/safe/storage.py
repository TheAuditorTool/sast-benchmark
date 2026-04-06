"""File storage helpers for the upload service.

This file is IDENTICAL between vuln/ and safe/ variants.
It provides path resolution and directory setup; the race window is in upload.py
where os.path.exists() check and the file write are separate operations.

Chain: concurrent POST /upload -> os.path.exists() check -> open(path, 'wb') write
Individual findings: TOCTOU on file existence check (medium)
Chain finding: concurrent upload overwrites another user's file (CWE-362, critical)
"""
import os

UPLOAD_DIR = "/tmp/uploads"


def ensure_upload_dir():
    os.makedirs(UPLOAD_DIR, exist_ok=True)


def resolve_path(filename):
    """Return the absolute upload path for filename (no directory traversal)."""
    safe_name = os.path.basename(filename)
    if not safe_name:
        return None
    return os.path.join(UPLOAD_DIR, safe_name)
