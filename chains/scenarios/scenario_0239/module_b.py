import os

DB_URL = os.environ.get("DB_URL", "postgresql://app:db-pass@localhost/sitedb")
SITEMAP_MAX_BYTES = 256 * 1024
ADMIN_TOKEN = os.environ.get("ADMIN_TOKEN", "admin-token-placeholder")
