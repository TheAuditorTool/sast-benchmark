"""Application configuration for jwt_secret_hardcoded scenario -- SAFE variant.

The JWT signing secret is loaded from an environment variable.
No secret is embedded in the source, so token forgery is not possible
from source-code access alone.

Chain broken: JWT_SECRET from env -> no hardcoded secret -> no token forgery
"""
import os

JWT_SECRET = os.environ.get("JWT_SECRET", "")
JWT_ALGORITHM = "HS256"
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
