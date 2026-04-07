import os

OAUTH_CLIENT_ID = os.environ.get("OAUTH_CLIENT_ID", "")
OAUTH_CLIENT_SECRET = os.environ.get("OAUTH_CLIENT_SECRET", "")
OAUTH_TOKEN_URL = "https://auth.example.com/oauth/token"
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
