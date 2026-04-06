"""Application configuration for oauth_client_secret scenario -- VULNERABLE variant.

The OAuth client secret is hardcoded. An attacker can use it with the
client ID to obtain access tokens and impersonate this application
against the OAuth provider.

Chain: hardcoded OAUTH_CLIENT_SECRET -> auth.py token exchange -> impersonation
Individual findings: hardcoded OAuth client secret (critical)
Chain finding: application impersonation via hardcoded OAuth client secret (critical)
"""

OAUTH_CLIENT_ID = "app-client-id-abc123"
OAUTH_CLIENT_SECRET = "oauth-secret-xyz789-do-not-commit"
OAUTH_TOKEN_URL = "https://auth.example.com/oauth/token"
SECRET_KEY = "dev-only-secret"
