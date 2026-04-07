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
    payload = base64.b64encode(json.dumps(claims).encode()).decode()
    sig = _sign(payload)
    return f"{payload}.{sig}"

def decrypt_claims(token: str) -> dict | None:
    if "." not in token:
        return None
    payload, sig = token.rsplit(".", 1)
    if not hmac.compare_digest(sig, _sign(payload)):
        return None
    try:
        return json.loads(base64.b64decode(payload))
    except Exception:
        return None
