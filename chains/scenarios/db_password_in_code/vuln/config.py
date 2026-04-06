"""Application configuration for db_password_in_code scenario -- VULNERABLE variant.

The database connection string with password is hardcoded. An attacker
who reads this file can connect to the production database directly.

Chain: hardcoded DB_URL -> auth.py connection -> unauthorized DB access
Individual findings: hardcoded database password (critical)
Chain finding: unauthorized database access via hardcoded connection string (critical)
"""

DB_URL = "postgresql://appuser:P@ssw0rd99@db.internal:5432/production"
SECRET_KEY = "dev-only-secret"
