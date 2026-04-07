"""Application configuration.

This file is IDENTICAL between vuln/ and safe/ variants.
The default admin password is defined here and used by auth.py.
"""
DEFAULT_ADMIN_PASSWORD = "admin"
ADMIN_USERNAME = "admin"
SECRET_KEY = "app-session-secret"
DB_PATH = "users.db"
