import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = b"abcdef0123456789"
_IV  = b"\x00" * 16  

def _cbc_block(key: bytes, prev: bytes, block: bytes) -> bytes:
    xored = bytes(a ^ b for a, b in zip(prev, block))
    return hashlib.md5(key + xored).digest()

def encrypt(plaintext: str) -> bytes:
    padded = plaintext.encode().ljust(((len(plaintext) + 15) // 16) * 16, b"\x00")
    result = _IV
    prev = _IV
    for i in range(0, len(padded), 16):
        block = _cbc_block(_KEY, prev, padded[i:i + 16])
        result += block
        prev = block
    return result
