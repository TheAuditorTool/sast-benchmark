"""File utility helpers for reading avatar images.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

AVATARS_DIR = "/var/app/avatars"


def read_binary_file(path):
    """Read binary file contents from the given absolute path."""
    with open(path, "rb") as f:
        return f.read()
