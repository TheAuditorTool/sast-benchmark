"""Crypto utilities -- SAFE variant.

Replaces the XOR cipher with HMAC-SHA256 signing so the token content
is authenticated and cannot be forged without the key.

Chain broken: HMAC-SHA256 -> XOR key recovery is irrelevant ->
              modified tokens produce invalid MACs -> forgery detected
"""
import hmac
import hashlib
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = os.environ.get("TOKEN_KEY", "safe-token-key-32-chars-here!!!").encode()


def encrypt(plaintext: str) -> bytes:
    """Sign plaintext with HMAC-SHA256; return payload|tag as bytes."""
    tag = hmac.new(_KEY, plaintext.encode(), hashlib.sha256).hexdigest()
    return f"{plaintext}|{tag}".encode()


def decrypt(ciphertext: bytes) -> str:
    """Verify HMAC tag and return plaintext, or raise ValueError."""
    raw = ciphertext.decode(errors="replace")
    if "|" not in raw:
        raise ValueError("Invalid token format")
    plaintext, tag = raw.rsplit("|", 1)
    expected = hmac.new(_KEY, plaintext.encode(), hashlib.sha256).hexdigest()
    if not hmac.compare_digest(tag, expected):
        raise ValueError("Token authentication failed")
    return plaintext
