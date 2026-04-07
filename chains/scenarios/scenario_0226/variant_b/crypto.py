import hmac
import hashlib
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = os.environ.get("TOKEN_KEY", "safe-token-key-32-chars-here!!!").encode()

def encrypt(plaintext: str) -> bytes:
    tag = hmac.new(_KEY, plaintext.encode(), hashlib.sha256).hexdigest()
    return f"{plaintext}|{tag}".encode()

def decrypt(ciphertext: bytes) -> str:
    raw = ciphertext.decode(errors="replace")
    if "|" not in raw:
        raise ValueError("Invalid token format")
    plaintext, tag = raw.rsplit("|", 1)
    expected = hmac.new(_KEY, plaintext.encode(), hashlib.sha256).hexdigest()
    if not hmac.compare_digest(tag, expected):
        raise ValueError("Token authentication failed")
    return plaintext
