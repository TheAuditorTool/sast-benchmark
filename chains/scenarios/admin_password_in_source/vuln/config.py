"""Application configuration for admin_password_in_source scenario -- VULNERABLE variant.

The admin password is hardcoded as a source-level constant. Anyone with
read access to the source code can obtain the credential and authenticate
as admin without any further privilege escalation.

Chain: hardcoded ADMIN_PASSWORD -> auth.py login check -> unauthorized admin access
Individual findings: hardcoded credential (critical)
Chain finding: unauthorized admin access via hardcoded password (critical)
"""

ADMIN_USERNAME = "admin"
ADMIN_PASSWORD = "SuperSecret123!"
SECRET_KEY = "dev-only-secret"
