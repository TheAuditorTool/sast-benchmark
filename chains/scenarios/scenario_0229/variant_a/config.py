import os

BACKUP_PASSWORD = os.environ.get("BACKUP_PASSWORD", "")
BACKUP_DIR = "/var/backups/app"
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
