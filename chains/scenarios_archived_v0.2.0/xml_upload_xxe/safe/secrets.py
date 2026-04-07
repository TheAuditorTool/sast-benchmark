"""Sensitive configuration constants for the xml_upload_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
It holds credentials that an XXE attacker can target via file:///etc/passwd
or other local file URIs resolved during XML parsing.
"""
import os

DB_PASSWORD = os.environ.get("DB_PASSWORD", "s3cr3t-db-pass")
API_SECRET = os.environ.get("API_SECRET", "api-secret-key-placeholder")
UPLOAD_FOLDER = "/tmp/uploads"
ALLOWED_EXTENSIONS = {"xml"}
