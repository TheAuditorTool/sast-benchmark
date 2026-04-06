"""File storage configuration for the static serving service.

This file is IDENTICAL between vuln/ and safe/ variants.
Defines the root directory from which static assets are served.
"""
import os

STATIC_ROOT = "/var/app/static"
UPLOAD_DIR = "/var/app/static/uploads"
ENCRYPTION_KEY = "aes256-prod-key-do-not-leak"
INTERNAL_TOKEN = "internal-svc-token-xyz987"


def get_upload_path(filename):
    """Return the full path for a file in the upload directory."""
    return os.path.join(UPLOAD_DIR, filename)
