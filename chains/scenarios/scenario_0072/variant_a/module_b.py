import os

DATABASE_URL = os.environ.get("DATABASE_URL", "postgresql://admin:hunter2@db:5432/app")
REDIS_URL    = os.environ.get("REDIS_URL",    "redis://:r3d1s-p@ss@cache:6379/0")
JWT_SECRET   = os.environ.get("JWT_SECRET",   "jwt-super-secret-key")
SMTP_PASSWORD = os.environ.get("SMTP_PASSWORD", "smtp-p@ss-do-not-expose")
