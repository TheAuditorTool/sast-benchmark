"""Configuration for the xliff_translation_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

TRANSLATION_UPLOAD_FOLDER = "/tmp/xliff_uploads"
ALLOWED_EXTENSIONS = {"xliff", "xlf"}
STRIPE_API_KEY = os.environ.get("STRIPE_API_KEY", "sk_test_placeholder")
TRANSLATION_API_KEY = os.environ.get("TRANSLATION_API_KEY", "trans-api-key-placeholder")
