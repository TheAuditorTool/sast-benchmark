"""Crypto utilities -- VULNERABLE variant.

Uses a static all-zero IV for every AES-CBC encryption.  A static IV
means identical plaintexts produce identical ciphertexts, and a known-IV
allows chosen-plaintext attacks that can reveal keystream information.

Chain: static IV -> identical ciphertexts for identical plaintexts ->
       attacker performs chosen-plaintext attack -> plaintext recovery /
       token forgery.
Individual findings: static/predictable IV (high)
Chain finding: predictable IV -> CBC chosen-plaintext -> token forge (high)
"""
import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = b"abcdef0123456789"
_IV  = b"\x00" * 16  # static IV -- never changes


def _cbc_block(key: bytes, prev: bytes, block: bytes) -> bytes:
    xored = bytes(a ^ b for a, b in zip(prev, block))
    return hashlib.md5(key + xored).digest()


def encrypt(plaintext: str) -> bytes:
    """Encrypt with AES-CBC using a static zero IV."""
    padded = plaintext.encode().ljust(((len(plaintext) + 15) // 16) * 16, b"\x00")
    result = _IV
    prev = _IV
    for i in range(0, len(padded), 16):
        block = _cbc_block(_KEY, prev, padded[i:i + 16])
        result += block
        prev = block
    return result
