"""Application configuration for the error stacktrace secret scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

SECRET_KEY = os.environ.get("SECRET_KEY", "dev-secret-do-not-use-in-prod")
STRIPE_API_KEY = os.environ.get("STRIPE_API_KEY", "sk_test_FAKEFAKEFAKEFAKE")
DATABASE_URL = os.environ.get("DATABASE_URL", "postgresql://admin:hunter2@db:5432/app")
