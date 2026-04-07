import json
import os

_raw = os.environ.get("SERVICE_ACCOUNT_KEY_JSON", "{}")
SERVICE_ACCOUNT_KEY = json.loads(_raw)
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
