"""Application configuration for admin_password_in_source scenario -- SAFE variant.

Credentials are read from environment variables at startup. No credential
is embedded in the source, so reading the code reveals nothing usable.

Chain broken: ADMIN_PASSWORD comes from env -> no hardcoded credential -> no unauthorized access
"""
import os

ADMIN_USERNAME = os.environ.get("ADMIN_USERNAME", "admin")
ADMIN_PASSWORD = os.environ.get("ADMIN_PASSWORD", "")
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
