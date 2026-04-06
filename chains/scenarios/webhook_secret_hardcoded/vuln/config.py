"""Application configuration for webhook_secret_hardcoded scenario -- VULNERABLE variant.

The webhook signing secret is hardcoded. An attacker can compute valid
HMAC signatures for arbitrary payloads and send forged webhooks that
the application will accept as legitimate.

Chain: hardcoded WEBHOOK_SECRET -> auth.py signature verification -> forged webhook accepted
Individual findings: hardcoded webhook signing secret (high)
Chain finding: webhook forgery via hardcoded signing secret (high)
"""

WEBHOOK_SECRET = "whsec_deadbeef1234567890abcdef"
SECRET_KEY = "dev-only-secret"
