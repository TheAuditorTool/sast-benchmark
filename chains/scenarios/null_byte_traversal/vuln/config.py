"""Application configuration for the file viewer service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
SECRET_KEY = "dev-secret-key-do-not-use-in-prod"
VIEWER_BASE_DIR = "/var/app/docs"
ADMIN_PASSWORD = "correct-horse-battery-staple"
SMTP_PASSWORD = "smtppass-prod-99"
