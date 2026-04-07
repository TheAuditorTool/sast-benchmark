"""Crypto utilities -- VULNERABLE variant.

Encrypts tokens with AES-ECB.  ECB encrypts each 16-byte block
independently, so identical plaintext blocks produce identical
ciphertext blocks.  An attacker can swap, copy, or replay ciphertext
blocks to manipulate the decrypted token content without knowing the key.

Chain: AES-ECB token issued -> attacker rearranges ciphertext blocks ->
       tampered token decrypts to attacker-chosen plaintext ->
       privilege escalation.
Individual findings: use of AES-ECB (high)
Chain finding: ECB block manipulation -> token forgery -> priv esc (critical)
"""
import os
import struct
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = b"0123456789abcdef"  # 16-byte AES-128 key


def _ecb_encrypt_block(key: bytes, block: bytes) -> bytes:
    """Encrypt a single 16-byte block with AES-ECB (pure stdlib via aes sim)."""
    # Use hashlib as a stand-in AES for demo purposes (real ECB weakness shown structurally)
    import hashlib
    return hashlib.md5(key + block).digest()


def encrypt_token(data: str) -> bytes:
    """Pad and encrypt data with AES-ECB (block-by-block)."""
    padded = data.encode().ljust(((len(data) + 15) // 16) * 16, b"\x00")
    result = b""
    for i in range(0, len(padded), 16):
        result += _ecb_encrypt_block(_KEY, padded[i:i + 16])
    return result


def decrypt_token(ciphertext: bytes) -> str:
    """Decrypt AES-ECB ciphertext (demo: reverses the block transform)."""
    return ciphertext.hex()
