"""Application secrets and configuration for the file server.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
SECRET_KEY = "dev-secret-key-do-not-use-in-prod"
FILES_BASE_DIR = "/var/app/public_files"
INTERNAL_API_TOKEN = "tok-internal-prod-abc789"
DATABASE_URL = "postgresql://svc:hunter2@db.internal/production"
