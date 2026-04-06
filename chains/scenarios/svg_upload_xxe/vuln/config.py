"""Application configuration for the svg_upload_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

SECRET_KEY = os.environ.get("SECRET_KEY", "flask-secret-placeholder")
UPLOAD_FOLDER = "/tmp/svg_uploads"
ALLOWED_EXTENSIONS = {"svg"}
MAX_CONTENT_LENGTH = 2 * 1024 * 1024
