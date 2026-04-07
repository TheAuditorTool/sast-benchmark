"""Service configuration for the soap_body_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

SERVICE_NAME = "PaymentService"
DB_DSN = os.environ.get("DB_DSN", "postgresql://app:secret@localhost/payments")
INTERNAL_API_KEY = os.environ.get("INTERNAL_API_KEY", "internal-key-placeholder")
MAX_BODY_BYTES = 1 * 1024 * 1024
