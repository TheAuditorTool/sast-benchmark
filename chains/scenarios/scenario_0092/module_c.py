import os

SMTP_HOST = "smtp.mailprovider.com"
SMTP_PORT = 587
SMTP_USER = os.environ.get("SMTP_USER", "")
SMTP_PASS = os.environ.get("SMTP_PASS", "")
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
