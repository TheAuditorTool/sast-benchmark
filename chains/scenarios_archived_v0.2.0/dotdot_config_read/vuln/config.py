"""Application configuration and secrets for the file download service.

This file is IDENTICAL between vuln/ and safe/ variants.
Stores the base download directory and application secret key.
"""
import os

SECRET_KEY = "dev-secret-key-do-not-use-in-prod"
DOWNLOAD_BASE_DIR = "/var/app/downloads"
DB_DSN = "postgresql://appuser:s3cr3tpassword@db.internal/appdb"
API_KEY = "prod-api-key-abc123xyz"
