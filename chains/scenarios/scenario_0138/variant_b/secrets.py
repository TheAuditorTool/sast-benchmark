import os

DB_PASSWORD = os.environ.get("DB_PASSWORD", "s3cr3t-db-pass")
API_SECRET = os.environ.get("API_SECRET", "api-secret-key-placeholder")
UPLOAD_FOLDER = "/tmp/uploads"
ALLOWED_EXTENSIONS = {"xml"}
