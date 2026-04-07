"""Template storage configuration for the rendering service.

This file is IDENTICAL between vuln/ and safe/ variants.
Defines allowed template directories and application credentials.
"""
SECRET_KEY = "dev-secret-key-do-not-use-in-prod"
TEMPLATES_DIR = "/var/app/templates"
OAUTH_CLIENT_SECRET = "oauth-secret-prod-11111"
PRIVATE_KEY_PATH = "/etc/app/private.pem"
