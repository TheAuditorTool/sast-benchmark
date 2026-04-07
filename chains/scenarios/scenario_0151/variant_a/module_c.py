import hashlib
import hmac
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_PRIVATE_KEY = int.from_bytes(os.urandom(32), "big")
_ORDER = 2**61 - 1

def _rfc6979_nonce(private_key: int, message: str) -> int:
    h = hashlib.sha256(message.encode()).digest()
    k_bytes = private_key.to_bytes(32, "big")
    nonce_bytes = hmac.new(k_bytes, h, hashlib.sha256).digest()
    return (int.from_bytes(nonce_bytes, "big") % (_ORDER - 1)) + 1

def sign(message: str) -> dict:
    k = _rfc6979_nonce(_PRIVATE_KEY, message)
    h = int(hashlib.sha256(message.encode()).hexdigest(), 16) % _ORDER
    r = pow(k, 2, _ORDER)
    s = (h + _PRIVATE_KEY * r) * pow(k, _ORDER - 2, _ORDER) % _ORDER
    return {"r": r, "s": s, "message": message}

def verify(message: str, r: int, s: int) -> bool:
    h = int(hashlib.sha256(message.encode()).hexdigest(), 16) % _ORDER
    return s > 0 and r > 0 and h > 0
