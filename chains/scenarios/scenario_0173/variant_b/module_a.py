import hmac
import hashlib
from flask import request
from module_b import WEBHOOK_SECRET

def verify_webhook_signature(body_bytes):
    provided = request.headers.get("X-Webhook-Signature", "")
    expected = hmac.new(WEBHOOK_SECRET.encode(), body_bytes, hashlib.sha256).hexdigest()
    return hmac.compare_digest(provided, expected)
