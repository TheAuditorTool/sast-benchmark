import os

APP_SECRET = os.environ.get("APP_SECRET", "app-secret-do-not-log")
DB_PASSWORD = os.environ.get("DB_PASSWORD", "db-p@ssw0rd-do-not-log")
LOG_FILE = "/tmp/app.log"
