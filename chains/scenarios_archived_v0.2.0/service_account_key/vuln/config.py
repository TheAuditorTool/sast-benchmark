"""Application configuration for service_account_key scenario -- VULNERABLE variant.

A service account private key JSON is hardcoded inline. An attacker
can extract this key and authenticate to cloud APIs as the service account,
with whatever permissions it has been granted.

Chain: hardcoded SERVICE_ACCOUNT_KEY -> auth.py API call -> cloud resource access
Individual findings: hardcoded service account private key (critical)
Chain finding: cloud resource access via hardcoded service account key (critical)
"""

SERVICE_ACCOUNT_KEY = {
    "type": "service_account",
    "project_id": "my-project-123",
    "private_key_id": "key-id-abc123",
    "private_key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA...(truncated)...\n-----END RSA PRIVATE KEY-----\n",
    "client_email": "app-svc@my-project-123.iam.gserviceaccount.com",
    "client_id": "123456789",
    "auth_uri": "https://accounts.google.com/o/oauth2/auth",
    "token_uri": "https://oauth2.googleapis.com/token",
}
SECRET_KEY = "dev-only-secret"
