"""Application configuration for api_key_in_config scenario -- SAFE variant.

The API key is read from an environment variable at startup.
No credential is embedded in the source.

Chain broken: API_KEY from env -> no hardcoded key -> no unauthorized API access
"""
import os

API_KEY = os.environ.get("API_KEY", "")
API_ENDPOINT = "https://api.example.com/v1"
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
