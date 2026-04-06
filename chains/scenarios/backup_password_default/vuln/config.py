"""Application configuration for backup_password_default scenario -- VULNERABLE variant.

The backup archive password is hardcoded as a default. An attacker can
use it to decrypt any backup produced by this application.

Chain: hardcoded BACKUP_PASSWORD -> auth.py backup creation -> encrypted archive decryptable
Individual findings: hardcoded backup password (high)
Chain finding: backup data exposure via hardcoded archive password (high)
"""

BACKUP_PASSWORD = "backup123"
BACKUP_DIR = "/var/backups/app"
SECRET_KEY = "dev-only-secret"
