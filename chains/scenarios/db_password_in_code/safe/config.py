"""Application configuration for db_password_in_code scenario -- SAFE variant.

The database URL is read from an environment variable. No password
is present in the source code.

Chain broken: DB_URL from env -> no hardcoded password -> no unauthorized DB access
"""
import os

DB_URL = os.environ.get("DATABASE_URL", "")
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
