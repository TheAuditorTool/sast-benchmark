"""Crypto utilities -- SAFE variant.

Uses HMAC-SHA256 for message integrity instead of CRC32.  HMAC requires
knowledge of the secret key to produce a valid MAC, preventing forgery.

Chain broken: HMAC-SHA256 -> attacker cannot produce valid MAC without
              the key -> message tampering is detected
"""
import hmac
import hashlib
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = os.environ.get("MSG_KEY", "integrity-secret-key-32-chars!!").encode()


def sign_message(payload: str) -> str:
    """Append HMAC-SHA256 tag to payload as hex."""
    tag = hmac.new(_KEY, payload.encode(), hashlib.sha256).hexdigest()
    return f"{payload}|{tag}"


def verify_message(signed: str) -> str | None:
    """Return payload if HMAC-SHA256 tag is valid, else None."""
    if "|" not in signed:
        return None
    payload, tag = signed.rsplit("|", 1)
    expected = hmac.new(_KEY, payload.encode(), hashlib.sha256).hexdigest()
    if hmac.compare_digest(tag, expected):
        return payload
    return None
