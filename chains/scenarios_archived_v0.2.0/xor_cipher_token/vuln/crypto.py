"""Crypto utilities -- VULNERABLE variant.

Encrypts tokens by XORing with a short repeating key ("xk").  A two-byte
repeating XOR key can be trivially broken by frequency analysis or by
XORing two ciphertexts together to cancel the key.

Chain: short XOR key -> key recovered via crib dragging or ciphertext XOR ->
       any token decrypted and re-encrypted with attacker-chosen content ->
       privilege escalation.
Individual findings: XOR cipher with short repeating key (critical)
Chain finding: XOR key recovery -> token content replaced -> priv esc (critical)
"""
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = b"xk"


def encrypt(plaintext: str) -> bytes:
    """XOR plaintext with repeating 2-byte key."""
    pt = plaintext.encode()
    return bytes(b ^ _KEY[i % len(_KEY)] for i, b in enumerate(pt))


def decrypt(ciphertext: bytes) -> str:
    """XOR ciphertext with repeating 2-byte key to recover plaintext."""
    return bytes(b ^ _KEY[i % len(_KEY)] for i, b in enumerate(ciphertext)).decode(errors="replace")
