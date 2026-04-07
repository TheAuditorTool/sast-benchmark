import os

WEBHOOK_SECRET = os.environ.get("WEBHOOK_SECRET", "")
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
