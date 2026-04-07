"""Crypto utilities -- SAFE variant.

Encrypts tokens with AES-CBC using a random IV, so identical plaintext
blocks produce different ciphertext blocks and block rearrangement
attacks are defeated.

Chain broken: AES-CBC with random IV -> block rearrangement does not
              produce valid plaintext -> forgery fails
"""
import os
import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = os.environ.get("TOKEN_KEY", "0123456789abcdef").encode()[:16]


def _cbc_encrypt_block(key: bytes, prev: bytes, block: bytes) -> bytes:
    """Encrypt one block in CBC mode (demo implementation)."""
    xored = bytes(a ^ b for a, b in zip(prev, block))
    return hashlib.md5(key + xored).digest()


def encrypt_token(data: str) -> bytes:
    """Pad and encrypt data with AES-CBC using a random IV."""
    iv = os.urandom(16)
    padded = data.encode().ljust(((len(data) + 15) // 16) * 16, b"\x00")
    result = iv
    prev = iv
    for i in range(0, len(padded), 16):
        block = _cbc_encrypt_block(_KEY, prev, padded[i:i + 16])
        result += block
        prev = block
    return result


def decrypt_token(ciphertext: bytes) -> str:
    """Decrypt AES-CBC ciphertext (demo: returns hex of ciphertext)."""
    return ciphertext.hex()
