import os

JWT_SECRET = os.environ.get("JWT_SECRET", "")
JWT_ALGORITHM = "HS256"
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")
