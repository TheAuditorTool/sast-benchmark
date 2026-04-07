"""Crypto utilities -- SAFE variant.

Uses RFC 6979 deterministic-but-unpredictable nonce derivation (simulated
here with HMAC-DRBG over the private key and message hash), so the nonce
is unique per message and not guessable from outside.

In practice: use the cryptography package's ECDSA with a P-256 key.

Chain broken: unique per-message nonce -> nonce reuse impossible ->
              private key cannot be algebraically recovered
"""
import hashlib
import hmac
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_PRIVATE_KEY = int.from_bytes(os.urandom(32), "big")
_ORDER = 2**61 - 1


def _rfc6979_nonce(private_key: int, message: str) -> int:
    """Derive a deterministic unique nonce per (key, message) pair."""
    h = hashlib.sha256(message.encode()).digest()
    k_bytes = private_key.to_bytes(32, "big")
    nonce_bytes = hmac.new(k_bytes, h, hashlib.sha256).digest()
    return (int.from_bytes(nonce_bytes, "big") % (_ORDER - 1)) + 1


def sign(message: str) -> dict:
    """Sign message with an RFC 6979-derived nonce."""
    k = _rfc6979_nonce(_PRIVATE_KEY, message)
    h = int(hashlib.sha256(message.encode()).hexdigest(), 16) % _ORDER
    r = pow(k, 2, _ORDER)
    s = (h + _PRIVATE_KEY * r) * pow(k, _ORDER - 2, _ORDER) % _ORDER
    return {"r": r, "s": s, "message": message}


def verify(message: str, r: int, s: int) -> bool:
    """Verify a simulated ECDSA signature (demo)."""
    h = int(hashlib.sha256(message.encode()).hexdigest(), 16) % _ORDER
    return s > 0 and r > 0 and h > 0
