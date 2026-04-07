import os

ENCRYPTION_KEY = os.environ.get("ENCRYPTION_KEY", "enc-key-placeholder")
OAUTH_CLIENT_SECRET = os.environ.get("OAUTH_CLIENT_SECRET", "oauth-secret-placeholder")
ENV_FILE_PATH = "/app/.env"
