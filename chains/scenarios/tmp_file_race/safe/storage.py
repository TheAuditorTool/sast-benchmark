"""Temporary file storage utilities -- SAFE variant.

Uses tempfile.mkstemp to atomically create a temp file with a random,
unpredictable name and return an open file descriptor. This eliminates
the TOCTOU window and prevents symlink races entirely.

Chain broken: mkstemp returns atomic fd -> no predictable path exists -> no symlink attack surface
"""
import tempfile


def get_temp_path():
    """Return an open file descriptor and path to a securely created temp file."""
    fd, path = tempfile.mkstemp(prefix="app_work_", suffix=".dat")
    return fd, path
