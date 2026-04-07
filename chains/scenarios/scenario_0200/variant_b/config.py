import os

UPLOAD_FOLDER = "/tmp/docx_uploads"
ALLOWED_EXTENSIONS = {"docx"}
MAX_DOCX_BYTES = 10 * 1024 * 1024
SIGNING_SECRET = os.environ.get("SIGNING_SECRET", "docx-signing-secret-placeholder")
