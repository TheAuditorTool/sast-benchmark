"""Application configuration for service_account_key scenario -- SAFE variant.

The service account key JSON is loaded from an environment variable at
runtime and parsed there. No private key is embedded in source.

Chain broken: SERVICE_ACCOUNT_KEY from env -> no hardcoded key -> no unauthorized cloud access
"""
import json
import os

_raw = os.environ.get("SERVICE_ACCOUNT_KEY_JSON", "{}")
SERVICE_ACCOUNT_KEY = json.loads(_raw)
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
