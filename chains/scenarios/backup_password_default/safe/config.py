"""Application configuration for backup_password_default scenario -- SAFE variant.

The backup password is loaded from an environment variable. No default
password is present in source; backups cannot be decrypted from code alone.

Chain broken: BACKUP_PASSWORD from env -> no hardcoded password -> no backup decryption
"""
import os

BACKUP_PASSWORD = os.environ.get("BACKUP_PASSWORD", "")
BACKUP_DIR = "/var/backups/app"
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
