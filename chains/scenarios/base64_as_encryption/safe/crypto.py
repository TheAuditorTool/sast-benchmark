"""Crypto utilities -- SAFE variant.

Signs claims with HMAC-SHA256 so the token cannot be modified without
invalidating the signature.  Base64 encoding is still used for transport
but the signature prevents forgery.

Chain broken: HMAC signature -> modified claims produce invalid signature ->
              forged tokens rejected
"""
import base64
import json
import hmac
import hashlib
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = os.environ.get("TOKEN_KEY", "safe-token-secret-key-32-chars!!").encode()


def _sign(data: str) -> str:
    return hmac.new(_KEY, data.encode(), hashlib.sha256).hexdigest()


def encrypt_claims(claims: dict) -> str:
    """Encode claims and append HMAC-SHA256 signature."""
    payload = base64.b64encode(json.dumps(claims).encode()).decode()
    sig = _sign(payload)
    return f"{payload}.{sig}"


def decrypt_claims(token: str) -> dict | None:
    """Verify signature then decode claims."""
    if "." not in token:
        return None
    payload, sig = token.rsplit(".", 1)
    if not hmac.compare_digest(sig, _sign(payload)):
        return None
    try:
        return json.loads(base64.b64decode(payload))
    except Exception:
        return None
