"""Secret values for the config_import_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
The XXE attack in parser.py targets file:///app/.env to exfiltrate
environment variable contents from the server.
"""
import os

ENCRYPTION_KEY = os.environ.get("ENCRYPTION_KEY", "enc-key-placeholder")
OAUTH_CLIENT_SECRET = os.environ.get("OAUTH_CLIENT_SECRET", "oauth-secret-placeholder")
ENV_FILE_PATH = "/app/.env"
