"""Application configuration for smtp_creds_hardcoded scenario -- SAFE variant.

SMTP credentials are loaded from environment variables. No password is
present in source, so an attacker cannot relay email from code access alone.

Chain broken: SMTP_PASS from env -> no hardcoded credential -> no unauthorized relay
"""
import os

SMTP_HOST = "smtp.mailprovider.com"
SMTP_PORT = 587
SMTP_USER = os.environ.get("SMTP_USER", "")
SMTP_PASS = os.environ.get("SMTP_PASS", "")
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
