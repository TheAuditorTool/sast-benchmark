"""Application configuration for webhook_secret_hardcoded scenario -- SAFE variant.

The webhook signing secret is loaded from an environment variable.
No secret is embedded in source, preventing signature forgery from code access.

Chain broken: WEBHOOK_SECRET from env -> no hardcoded secret -> no webhook forgery
"""
import os

WEBHOOK_SECRET = os.environ.get("WEBHOOK_SECRET", "")
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
