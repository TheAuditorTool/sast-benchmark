"""Temporary file storage utilities -- VULNERABLE variant.

Constructs a predictable temp file path using only the process PID.
Between the path construction and the file open there is a TOCTOU window
where an attacker can create a symlink at that path pointing to an
arbitrary target, causing the subsequent write to clobber that target.

Chain: predictable temp path created -> attacker places symlink -> write follows symlink -> file tampered
Individual findings: insecure temp file creation (CWE-732)
Chain finding: symlink-assisted file tampering via predictable temp path (critical)
"""
import os


def get_temp_path():
    """Return a predictable temporary file path based on PID."""
    return "/tmp/app_work_{}.dat".format(os.getpid())
