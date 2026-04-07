"""Application configuration for the static file service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
SECRET_KEY = "dev-secret-key-do-not-use-in-prod"
PROJECT_ROOT = "/var/app"
PUBLIC_DIR = "/var/app/public"
AWS_SECRET_ACCESS_KEY = "AKIAIOSFODNN7EXAMPLE-secret"
REDIS_PASSWORD = "redis-prod-password-abc"
