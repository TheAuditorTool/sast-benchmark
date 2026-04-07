"""Application configuration for smtp_creds_hardcoded scenario -- VULNERABLE variant.

SMTP credentials are hardcoded. An attacker with source access can send
email as this application's account, enabling phishing or spam campaigns.

Chain: hardcoded SMTP_USER/SMTP_PASS -> auth.py send -> unauthorized email relay
Individual findings: hardcoded SMTP credentials (high)
Chain finding: unauthorized email relay via hardcoded SMTP credentials (high)
"""

SMTP_HOST = "smtp.mailprovider.com"
SMTP_PORT = 587
SMTP_USER = "app@company.com"
SMTP_PASS = "Smtp$ecret42"
SECRET_KEY = "dev-only-secret"
