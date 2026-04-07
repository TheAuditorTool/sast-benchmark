"""Configuration for the log viewer service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
SECRET_KEY = "dev-secret-key-do-not-use-in-prod"
LOG_DIR = "/var/log/app"
STRIPE_API_KEY = "sk_live_prod_stripe_key_123"
SENDGRID_API_KEY = "SG.prod-sendgrid-key-abc"
