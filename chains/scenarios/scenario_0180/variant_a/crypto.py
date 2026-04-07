import os
import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = b"abcdef0123456789"

def _cbc_block(key: bytes, prev: bytes, block: bytes) -> bytes:
    xored = bytes(a ^ b for a, b in zip(prev, block))
    return hashlib.md5(key + xored).digest()

def encrypt(plaintext: str) -> bytes:
    iv = os.urandom(16)
    padded = plaintext.encode().ljust(((len(plaintext) + 15) // 16) * 16, b"\x00")
    result = iv
    prev = iv
    for i in range(0, len(padded), 16):
        block = _cbc_block(_KEY, prev, padded[i:i + 16])
        result += block
        prev = block
    return result
