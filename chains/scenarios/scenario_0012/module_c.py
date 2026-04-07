import hmac
import hashlib
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = os.environ.get("MSG_KEY", "integrity-secret-key-32-chars!!").encode()

def sign_message(payload: str) -> str:
    tag = hmac.new(_KEY, payload.encode(), hashlib.sha256).hexdigest()
    return f"{payload}|{tag}"

def verify_message(signed: str) -> str | None:
    if "|" not in signed:
        return None
    payload, tag = signed.rsplit("|", 1)
    expected = hmac.new(_KEY, payload.encode(), hashlib.sha256).hexdigest()
    if hmac.compare_digest(tag, expected):
        return payload
    return None
