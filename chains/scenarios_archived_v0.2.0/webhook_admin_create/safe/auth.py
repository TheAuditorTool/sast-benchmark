"""Webhook signature verification -- VULNERABLE variant.

verify_webhook_signature() is defined but the admin creation route does
not call it. Any unauthenticated POST to /webhooks/provision can trigger
admin user creation without a valid HMAC signature.

Chain: unauthenticated POST -> signature check absent -> admin user created
Individual findings: missing authorization check (CWE-862)
Chain finding: unauthed webhook triggers admin account creation
"""
import hmac
import hashlib
from flask import request
from models import WEBHOOK_SECRET


def verify_webhook_signature(body_bytes):
    """Return True if the X-Webhook-Signature header matches the expected HMAC.

    VULNERABLE: this function exists but is never called from the route.
    """
    provided = request.headers.get("X-Webhook-Signature", "")
    expected = hmac.new(WEBHOOK_SECRET.encode(), body_bytes, hashlib.sha256).hexdigest()
    return hmac.compare_digest(provided, expected)
