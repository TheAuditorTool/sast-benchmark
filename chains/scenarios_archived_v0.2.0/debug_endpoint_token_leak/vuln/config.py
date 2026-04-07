"""Application configuration for the debug token leak scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

SECRET_KEY = os.environ.get("SECRET_KEY", "dev-secret-do-not-use-in-prod")
DEBUG = os.environ.get("DEBUG", "true").lower() == "true"

# Simulated in-memory session store: token -> user_id
SESSION_STORE = {
    "tok-alice-abc123": "u1",
    "tok-admin-xyz999": "admin",
}
