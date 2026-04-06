"""Configuration for the xslt_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

TRANSFORM_MAX_BYTES = 128 * 1024
SERVER_KEY_FILE = "/etc/ssl/private/server.key"
APP_SECRET = os.environ.get("APP_SECRET", "xslt-app-secret-placeholder")
