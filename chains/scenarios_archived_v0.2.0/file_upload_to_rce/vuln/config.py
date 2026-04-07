"""Application configuration for plugin upload service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

UPLOAD_DIR = os.path.join(os.path.dirname(__file__), "plugins")
MAX_UPLOAD_SIZE = 5 * 1024 * 1024  # 5 MB
ALLOWED_MIME_TYPES = frozenset([
    "application/zip",
    "application/x-python-code",
    "text/x-python",
    "application/octet-stream",
])

os.makedirs(UPLOAD_DIR, exist_ok=True)
