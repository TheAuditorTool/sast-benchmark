"""Configuration for the sitemap_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

DB_URL = os.environ.get("DB_URL", "postgresql://app:db-pass@localhost/sitedb")
SITEMAP_MAX_BYTES = 256 * 1024
ADMIN_TOKEN = os.environ.get("ADMIN_TOKEN", "admin-token-placeholder")
