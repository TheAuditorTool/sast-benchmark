"""Application configuration for oauth_client_secret scenario -- SAFE variant.

The OAuth client secret is loaded from an environment variable.
No secret is embedded in source, preventing impersonation from code access.

Chain broken: OAUTH_CLIENT_SECRET from env -> no hardcoded secret -> no impersonation
"""
import os

OAUTH_CLIENT_ID = os.environ.get("OAUTH_CLIENT_ID", "")
OAUTH_CLIENT_SECRET = os.environ.get("OAUTH_CLIENT_SECRET", "")
OAUTH_TOKEN_URL = "https://auth.example.com/oauth/token"
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
